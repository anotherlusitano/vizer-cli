use inquire::{InquireError, Select};

use crate::VIM_MODE;

pub fn choose_episode(mut episodes: Vec<String>) -> Result<String, ()> {
    print!("\x1B[2J\x1B[1;1H");

    let help_msg = format!("Total of episodes to watch: {}", episodes.len());

    // the episodes start at zero
    episodes = episodes
        .clone()
        .into_iter()
        .map(|e| (e.parse::<u32>().unwrap() + 1).to_string())
        .collect();

    let ans: Result<String, InquireError> =
        Select::new("Select the episode you want to watch:", episodes)
            .with_help_message(&help_msg)
            .with_page_size(25)
            .with_vim_mode(unsafe { VIM_MODE })
            .prompt();

    match ans {
        Ok(choice) => Ok(choice),
        Err(_) => Err(println!("There was an error, please try again")),
    }
}
