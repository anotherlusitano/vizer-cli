use fantoccini::{error::CmdError, Client, Locator};

use crate::season::Season;

pub async fn parse_seasons(driver: &Client) -> Result<Vec<Season>, CmdError> {
    let season_css_selector = "div[data-season-id]";

    driver
        .wait()
        .for_element(Locator::Css(season_css_selector))
        .await?;
    let season_items = driver.find_all(Locator::Css(season_css_selector)).await?;
    let mut seasons = Vec::new();

    for season_element in season_items {
        let season = Season {
            text: season_element.html(true).await?,
            web_element: season_element,
        };

        seasons.push(season);
    }

    Ok(seasons)
}
