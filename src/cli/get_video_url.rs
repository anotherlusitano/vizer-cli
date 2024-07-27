use fantoccini::{error::CmdError, Client, Locator};

use crate::{driver::click_element::click_element, TRANSLATION};

pub async fn get_video_url(driver: &Client, media_url: String) -> Result<String, CmdError> {
    let language = TRANSLATION.get().unwrap();

    driver.goto(&media_url).await?;

    driver.enter_frame(Some(0)).await?;

    // we have to wait for the play button to appear
    driver
        .wait()
        .for_element(Locator::Css(".vjs-big-play-button"))
        .await?;
    let play_button = driver.find(Locator::Css(".vjs-big-play-button")).await?;

    click_element(driver, play_button, language.click_episode_err).await?;

    let video = driver.find(Locator::Css("#videojs_html5_api")).await?;

    let video_url = format!("https:{}", video.attr("src").await?.unwrap());

    Ok(video_url)
}
