use fantoccini::{error::CmdError, Client, Locator};

use crate::season::Season;

pub async fn parse_seasons(driver: &Client) -> Result<Vec<Season>, CmdError> {
    let season_items = driver.find_all(Locator::Css("div[data-season-id]")).await?;
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
