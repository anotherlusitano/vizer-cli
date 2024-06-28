use selthi::Select;

use crate::{TRANSLATION, VIM_MODE};

pub fn choose_episode_with_images(
    episodes: Vec<String>,
    images_path: Vec<String>,
) -> Result<usize, ()> {
    let language = TRANSLATION.get().unwrap();
    let vim_mode = VIM_MODE.get().unwrap();
    print!("\x1B[2J\x1B[1;1H");

    let help_msg = format!("{} {}", language.total_episode_misc_text, episodes.len());

    let images_path = images_path.iter().map(String::as_str).collect();
    let episodes = episodes.iter().map(String::as_str).collect();

    let ans = Select::new(language.select_episode_misc_text, episodes)
        .with_help_message(&help_msg)
        .with_page_size(25)
        .with_vim_mode(*vim_mode)
        .with_images(images_path)
        .prompt();

    match ans {
        Some(choice) => {
            let mut episode_number = choice.split_whitespace();

            let episode: usize = episode_number.next().unwrap().parse::<usize>().unwrap() - 1;

            Ok(episode)
        }
        None => Err(println!("{}", language.choose_episode_err)),
    }
}
