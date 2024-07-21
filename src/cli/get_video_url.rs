use thirtyfour::prelude::*;

use crate::{driver::click_element::click_element, TRANSLATION};

pub async fn get_video_url(driver: &WebDriver, media_url: String) -> WebDriverResult<String> {
    let language = TRANSLATION.get().unwrap();

    driver.goto(media_url).await?;

    driver.enter_frame(0).await?;

    let play_button = driver
        .query(By::ClassName("vjs-big-play-button"))
        .first()
        .await?;

    click_element(driver, play_button, language.click_episode_err).await?;

    let video = driver.find(By::Id("videojs_html5_api")).await?;

    let video_url = format!("https:{}", video.attr("src").await?.unwrap());

    Ok(video_url)
}
