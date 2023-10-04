use std::{process::Command, thread::sleep, time::Duration};

use thirtyfour::prelude::*;

use crate::{
    cli::{choose_episode::choose_episode, choose_lang::choose_lang, choose_season::choose_season},
    media::Media,
};

#[tokio::main]
pub async fn watch_media(media: Media) -> WebDriverResult<()> {
    let url = format!("https://vizer.in/{}", &media.link);

    let mut chromedriver = Command::new("chromedriver").spawn().unwrap();
    // we need to wait chromedriver to start :(
    sleep(Duration::from_millis(100));

    clearscreen::clear().unwrap();
    println!("Preparing everything, which can take a while");

    let mut caps = DesiredCapabilities::chrome();
    caps.set_headless().unwrap();
    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    driver.goto(url).await?;

    if media.link.contains("serie/") {
        let season_items = driver.find_all(By::Css("div[data-season-id]")).await?;

        let mut season_opts: Vec<String> = Vec::new();

        for season in season_items {
            season_opts.push(season.inner_html().await?);
        }

        let season_opt = if season_opts.len() > 1 {
            choose_season(season_opts).unwrap()
        } else {
            season_opts[0].to_string()
        };

        let season_btn_xpath = format!("//div[@data-season-id and text()='{}']", season_opt);
        let season_element = driver.query(By::XPath(&season_btn_xpath)).first().await?;

        // we execute a js script to not be redirect to other page by the pop up
        driver
            .execute(
                r#"
            arguments[0].click();
            "#,
                vec![season_element.to_json()?],
            )
            .await
            .expect("Error: Can't click on the season");

        println!("Getting episodes");

        let episodes_list = driver.query(By::Id("episodesList")).first().await?;
        episodes_list.wait_until().displayed().await?;

        let episodes_items = episodes_list
            .find_all(By::ClassName("bslider-item"))
            .await?;

        let mut episode_opts: Vec<String> = Vec::new();

        for episode in &episodes_items {
            episode_opts.push(
                episode
                    .find(By::Css("div[slide-number]"))
                    .await?
                    .attr("slide-number")
                    .await?
                    .unwrap(),
            );
        }

        let episode_opt: String = if episode_opts.len() > 1 {
            (choose_episode(episode_opts)
                .unwrap()
                .parse::<u32>()
                .unwrap()
                - 1)
            .to_string()
        } else {
            episode_opts[0].to_string()
        };

        let episode_btn_css = format!(r#"div[slide-number="{}"]"#, episode_opt);

        let episode_element = driver.find(By::Css(&episode_btn_css)).await?;

        // we execute a js script to not be redirect to other page by the pop up
        driver
            .execute(
                r#"
            arguments[0].click();
            "#,
                vec![episode_element.to_json()?],
            )
            .await
            .expect("Error: Can't click on the episode");
    }

    println!("Getting languages options");

    let langs_items = driver
        .query(By::Css("div[data-load-video-players]"))
        .all()
        .await?;

    let mut langs_opts: Vec<String> = Vec::new();

    for lang in &langs_items {
        langs_opts.push(lang.inner_html().await?);
    }

    let lang_opt = if langs_opts.len() == 2 {
        choose_lang(langs_opts).unwrap()
    } else {
        langs_opts[0].to_string()
    };

    let lang_btn_xpath = format!("//div[text()='{}']", lang_opt);

    let lang_element = driver.find(By::XPath(&lang_btn_xpath)).await?;

    // we execute a js script to not be redirect to other page by the pop up
    driver
        .execute(
            r#"
            arguments[0].click();
            "#,
            vec![lang_element.to_json()?],
        )
        .await?;

    let mut media_id: Option<String> = None;
    for lang in &langs_items {
        if lang.inner_html().await? == lang_opt {
            media_id = lang.attr("data-load-video-players").await?;
        }
    }

    println!("Fetching service");

    let media_url = format!(
        "https://vizer.in/embed/getEmbed.php?id={}&sv=mixdrop",
        media_id.unwrap()
    );

    driver.goto(media_url).await?;

    driver.enter_frame(0).await?;

    let play_button = driver
        .query(By::ClassName("vjs-big-play-button"))
        .first()
        .await?;

    // we execute a js script to not be redirect to other page by the pop up
    driver
        .execute(
            r#"
            arguments[0].click();
            "#,
            vec![play_button.to_json()?],
        )
        .await?;

    let video = driver.find(By::Id("videojs_html5_api")).await?;

    let video_url = format!("https:{}", video.attr("src").await?.unwrap());

    driver.quit().await?;
    chromedriver.kill().unwrap();

    open_vlc(&video_url);

    Ok(())
}

fn open_vlc(video_url: &str) {
    println!("Starting the player");

    let output = Command::new("vlc")
        .args(["--fullscreen", "--play-and-exit", video_url])
        .spawn();

    match output {
        Ok(mut child) => match child.wait() {
            Ok(status) => {
                if status.success() {
                    clearscreen::clear().unwrap();
                    println!("VLC exited successfully");
                } else {
                    println!("VLC exited with an error: {:?}", status.code());
                }
            }
            Err(err) => {
                println!("Failed to wait for VLC: {}", err);
            }
        },
        Err(err) => {
            println!("Failed to start VLC: {}", err);
        }
    }
}
