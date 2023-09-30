use std::{process::Command, thread::sleep, time::Duration};

use thirtyfour::prelude::*;

use crate::{cli::choose_lang::choose_lang, media::Media};

#[tokio::main]
pub async fn watch_media(media: Media) -> WebDriverResult<()> {
    let url = format!("https://vizer.in/{}", &media.link);

    let mut chromedriver = Command::new("chromedriver").spawn().unwrap();
    // we need to wait chromedriver to start :(
    sleep(Duration::from_millis(100));

    clearscreen::clear().unwrap();
    println!("Getting languages options");

    let mut caps = DesiredCapabilities::chrome();
    caps.set_headless().unwrap();
    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    driver.goto(url).await?;

    let langs_table = driver.find(By::ClassName("langs")).await?;
    let langs_items = langs_table.find_all(By::ClassName("item")).await?;

    let mut langs_opts: Vec<String> = Vec::new();

    for lang in &langs_items {
        langs_opts.push(lang.text().await?);
    }

    let lang_opt = if langs_opts.len() == 2 {
        choose_lang(langs_opts).unwrap()
    } else {
        langs_opts[0].to_string()
    };

    let lang_btn_xpath = format!("//div[text()='{}']", lang_opt);

    let lang_element = driver.find(By::XPath(&lang_btn_xpath)).await?;

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
        if lang.text().await? == lang_opt {
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
    println!("Starting player");

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
