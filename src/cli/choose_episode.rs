use inquire::{InquireError, Select};

use crate::VIM_MODE;

pub fn choose_episode(episodes: Vec<String>) -> Result<usize, ()> {
    print!("\x1B[2J\x1B[1;1H");

    let help_msg = format!("Total of episodes to watch: {}", episodes.len());

    let ans: Result<String, InquireError> =
        Select::new("Select the episode you want to watch:", episodes)
            .with_help_message(&help_msg)
            .with_page_size(25)
            .with_vim_mode(unsafe { VIM_MODE })
            .prompt();

    match ans {
        Ok(choice) => {
            let mut episode_number = choice.split_whitespace();

            let episode: usize = episode_number.next().unwrap().parse::<usize>().unwrap() - 1;

            Ok(episode)
        }
        Err(_) => Err(println!("There was an error, please try again")),
    }
}
