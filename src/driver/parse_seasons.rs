use thirtyfour::prelude::*;

use crate::season::Season;

pub async fn parse_seasons(driver: &WebDriver) -> WebDriverResult<Vec<Season>> {
    let season_items = driver.find_all(By::Css("div[data-season-id]")).await?;
    let mut seasons = Vec::new();

    for season_element in season_items {
        let season = Season {
            text: season_element.inner_html().await?,
            web_element: season_element,
        };

        seasons.push(season);
    }

    Ok(seasons)
}
