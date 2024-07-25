use selthi::Select;

use crate::{episode::Episode, season::Season};

pub fn menu(menu_options: Vec<&str>) -> Result<&str, ()> {
    let ans = Select::new("Select what you want to do", menu_options)
        .without_help_message()
        .prompt();

    match ans {
        Some(option) => Ok(option),
        None => Err(println!("Couldn't get option!")),
    }
}

pub fn get_menu_options(
    seasons: &[Season],
    episodes: &[Episode],
    current_episode: usize,
) -> Vec<&'static str> {
    if seasons.is_empty() {
        // Will return this options when its a movie
        return vec!["replay", "search", "quit"];
    }
    let mut menu_options: Vec<&str> = Vec::new();

    let first_episode = episodes.first().unwrap();
    let last_episode = episodes.last().unwrap();

    let is_last_episode = current_episode == last_episode.episode_number;
    let is_first_episode = current_episode == first_episode.episode_number;

    let is_just_one_episode = episodes.len() == 1;
    let is_just_one_season = seasons.len() == 1;

    if !is_last_episode {
        menu_options.push("next");
    }

    menu_options.push("replay");

    if !is_first_episode {
        menu_options.push("previous");
    }

    menu_options.push("search");

    if !is_just_one_episode {
        menu_options.push("select episode");
    }

    if !is_just_one_season {
        menu_options.push("select season");
    }

    menu_options.push("quit");

    menu_options
}
