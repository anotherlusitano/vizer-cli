use thirtyfour::prelude::*;

pub async fn get_video_url(driver: &WebDriver, media_url: String) -> WebDriverResult<String> {
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

    Ok(video_url)
}
