use selthi::Select;
use thirtyfour::prelude::*;

use crate::{
    driver::click_element::click_element, fs::posters::get_posters_path, TRANSLATION, VIM_MODE,
};

pub async fn choose_episode(driver: &WebDriver, img_mode: bool) -> WebDriverResult<()> {
    let episodes_list = driver.find(By::ClassName("episodes")).await?;

    let episodes_items = episodes_list.query(By::ClassName("item")).all().await?;

    let mut episodes_opt: Vec<String> = Vec::new();
    let mut episodes_img_url: Vec<String> = Vec::new();

    for (i, item) in episodes_items.iter().enumerate() {
        if item.class_name().await?.unwrap() != "item unreleased " {
            let episode_text = item.find(By::Tag("span")).await?.inner_html().await?;

            // this thing of adding by 1
            // is just to show the episodes starting in 1 instead of 0
            let episode: String = format!("{} - {}", i + 1, episode_text);

            episodes_opt.push(episode);

            if img_mode {
                let img_src = item.find(By::Tag("img")).await?.attr("src").await?.unwrap();
                let img_url = format!("https://vizertv.in{}", img_src.replace("s/185", "s/500"));
                episodes_img_url.push(img_url);
            }
        }
    }

    let language = TRANSLATION.get().unwrap();

    let episode_opt: usize = if episodes_opt.len() > 1 {
        let episodes_opt = episodes_opt.iter().map(String::as_str).collect();

        match img_mode {
            true => {
                let posters_path = get_posters_path(episodes_img_url).await.unwrap();
                let posters_path = posters_path.iter().map(String::as_str).collect();

                get_episode(episodes_opt, Some(posters_path)).unwrap()
            }
            false => get_episode(episodes_opt, None).unwrap(),
        }
    } else {
        episodes_opt[0].parse::<usize>().unwrap() - 1
    };

    click_element(
        driver,
        episodes_items[episode_opt].clone(),
        language.click_episode_err,
    )
    .await?;

    Ok(())
}

fn get_episode(episodes: Vec<&str>, images_path: Option<Vec<&str>>) -> Result<usize, ()> {
    let language = TRANSLATION.get().unwrap();
    let vim_mode = VIM_MODE.get().unwrap();
    print!("\x1B[2J\x1B[1;1H");

    let help_msg = format!("{} {}", language.total_episode_misc_text, episodes.len());

    let ans = if let Some(images_path) = images_path {
        Select::new(language.select_episode_misc_text, episodes)
            .with_help_message(&help_msg)
            .with_page_size(25)
            .with_vim_mode(*vim_mode)
            .with_images(images_path)
            .prompt()
    } else {
        Select::new(language.select_episode_misc_text, episodes)
            .with_help_message(&help_msg)
            .with_page_size(25)
            .with_vim_mode(*vim_mode)
            .prompt()
    };

    match ans {
        Some(choice) => {
            let mut episode_number = choice.split_whitespace();

            let episode: usize = episode_number.next().unwrap().parse::<usize>().unwrap() - 1;

            Ok(episode)
        }
        None => Err(println!("{}", language.choose_episode_err)),
    }
}
