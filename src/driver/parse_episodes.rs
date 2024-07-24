use thirtyfour::prelude::*;

use crate::{episode::Episode, fs::posters::get_posters_path};

pub async fn parse_episodes(driver: &WebDriver, img_mode: bool) -> WebDriverResult<Vec<Episode>> {
    let episodes_list = driver.find(By::ClassName("episodes")).await?;

    let episodes_items = episodes_list.query(By::ClassName("item")).all().await?;

    let mut episodes = Vec::new();

    for (i, episode_element) in episodes_items.iter().enumerate() {
        if episode_element.class_name().await?.unwrap() != "item unreleased " {
            let episode_text = episode_element
                .find(By::Tag("span"))
                .await?
                .inner_html()
                .await?;

            // this thing of adding by 1
            // is just to show the episodes starting in 1 instead of 0
            let text: String = format!("{} - {}", i + 1, episode_text);

            let mut img_path = None;

            if img_mode {
                let img_src = episode_element
                    .find(By::Tag("img"))
                    .await?
                    .attr("src")
                    .await?
                    .unwrap();
                let img_url = format!("https://vizertv.in{}", img_src.replace("s/185", "s/500"));

                let vec_of_img_url: Vec<String> = vec![img_url];
                let poster_path = get_posters_path(vec_of_img_url).await.unwrap()[0].clone();
                img_path = Some(poster_path);
            }

            let episode = Episode {
                text,
                img_path,
                web_element: episode_element.to_owned(),
            };

            episodes.push(episode);
        }
    }

    Ok(episodes)
}
