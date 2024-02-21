use std::{process::Command, thread::sleep, time::Duration};

use thirtyfour::prelude::*;

use crate::{
    cli::{choose_episode::choose_episode, choose_lang::choose_lang, choose_season::choose_season},
    media::Media,
    player::vlc::open_vlc,
};

#[tokio::main]
pub async fn watch_media(media: Media) -> WebDriverResult<()> {
    let url = format!("https://vizer.in/{}", &media.link);

    let mut chromedriver = Command::new("chromedriver").spawn().unwrap();
    // we need to wait chromedriver to start :(
    sleep(Duration::from_millis(100));

    print!("\x1B[2J\x1B[1;1H");
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

        let season_btn_xpath = format!("//div[text()='{}']", season_opt);
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

        let episodes_list = driver.find(By::ClassName("episodes")).await?;

        let episodes_items = episodes_list.query(By::ClassName("item")).all().await?;

        let mut episode_opts: Vec<String> = Vec::new();

        for i in 0..episodes_items.len() {
            // this thing of adding by 1
            // is just to show the episodes starting in 1
            episode_opts.push((i + 1).to_string());
        }

        let episode_opt: usize = if episode_opts.len() > 1 {
            choose_episode(episode_opts)
                .unwrap()
                .parse::<usize>()
                .unwrap()
                - 1
        } else {
            episode_opts[0].parse::<usize>().unwrap() - 1
        };

        // we execute a js script to not be redirect to other page by the pop up
        driver
            .execute(
                r#"
            arguments[0].click();
            "#,
                vec![episodes_items[episode_opt].to_json()?],
            )
            .await
            .expect("Error: Can't click on the episode");
    }

    println!("Getting languages options");

    let langs_items = driver.query(By::Css("div[data-audio]")).all().await?;

    let mut langs_opts: Vec<String> = Vec::new();

    for lang in &langs_items {
        let opt = lang
            .attr("data-audio")
            .await?
            .expect("Couldn't retrieve languages options.");
        langs_opts.push(opt);
    }

    let lang_opt = if langs_opts.len() == 2 {
        choose_lang(langs_opts.clone()).unwrap()
    } else {
        langs_opts[0].to_string()
    };

    let mut media_id: Option<String> = None;
    for i in 0..langs_opts.len() {
        if langs_opts[i] == lang_opt {
            media_id = langs_items[i].attr("data-load-player").await?;
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
