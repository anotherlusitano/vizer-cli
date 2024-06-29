use thirtyfour::prelude::*;

use crate::{
    cli::{
        choose_episode::choose_episode, choose_season::choose_season, get_media_url::get_media_url,
    },
    driver::start_driver::{get_driver, start_browser_driver},
    media::Media,
    player::{mpv::open_mpv, vlc::open_vlc},
    TRANSLATION, USE_MPV,
};

#[tokio::main]
pub async fn watch_media(media: Media, img_mode: bool) -> WebDriverResult<()> {
    let language = TRANSLATION.get().unwrap();
    let use_mpv = USE_MPV.get().unwrap();

    print!("\x1B[2J\x1B[1;1H");
    println!("{}", language.preparing_misc_text);

    let mut browser_driver = start_browser_driver();

    let driver = get_driver().await;

    let url = format!("https://vizer.in/{}", &media.url);
    driver.goto(url).await?;

    if media.url.contains("serie/") {
        choose_season(&driver).await?;

        println!("{}", language.getting_episodes_misc_text);

        choose_episode(&driver, img_mode).await?;
    }

    let media_url = get_media_url(&driver).await?;

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
    browser_driver.kill().unwrap();

    if *use_mpv {
        open_mpv(&video_url);
    } else {
        open_vlc(&video_url);
    }

    Ok(())
}
