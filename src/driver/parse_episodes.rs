use fantoccini::{error::CmdError, Client, Locator};

use crate::{episode::Episode, fs::posters::get_posters_path};

pub async fn parse_episodes(driver: &Client, img_mode: bool) -> Result<Vec<Episode>, CmdError> {
    let episodes_css_selector = ".episodes > div.item:not(.unreleased)";

    // we have to wait for the episodes to appear
    driver
        .wait()
        .for_element(Locator::Css(episodes_css_selector))
        .await?;

    let episodes_items = driver.find_all(Locator::Css(episodes_css_selector)).await?;

    let mut episodes = Vec::new();
    let mut list_of_images_url = Vec::new();
    let mut list_of_episodes_text = Vec::new();
    let mut list_of_episodes_elements = Vec::new();

    for (i, episode_element) in episodes_items.iter().enumerate() {
        let episode_text = episode_element
            .find(Locator::Css("span"))
            .await?
            .html(true)
            .await?;

        // this thing of adding by 1
        // is just to show the episodes starting in 1 instead of 0
        let text: String = format!("{} - {}", i + 1, episode_text);

        list_of_episodes_text.push(text);
        list_of_episodes_elements.push(episode_element);

        if img_mode {
            let img_src = episode_element
                .find(Locator::Css("img"))
                .await?
                .attr("src")
                .await?
                .unwrap();
            let img_url = format!("https://vizertv.in{}", img_src.replace("s/185", "s/500"));
            list_of_images_url.push(img_url);
        }
    }

    let poster_paths = if img_mode {
        Some(get_posters_path(list_of_images_url).await.unwrap())
    } else {
        None
    };

    for episode in 0..list_of_episodes_elements.len() {
        let episode = Episode {
            text: list_of_episodes_text[episode].clone(),
            img_path: poster_paths.as_ref().map(|paths| paths[episode].clone()),
            episode_number: episode,
            web_element: list_of_episodes_elements[episode].to_owned(),
        };

        episodes.push(episode);
    }

    Ok(episodes)
}
