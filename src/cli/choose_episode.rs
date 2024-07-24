use selthi::Select;

use crate::{TRANSLATION, VIM_MODE};

pub fn choose_episode(episodes: Vec<&str>, images_path: Option<Vec<&str>>) -> Result<usize, ()> {
    let language = TRANSLATION.get().unwrap();
    let vim_mode = VIM_MODE.get().unwrap();

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
