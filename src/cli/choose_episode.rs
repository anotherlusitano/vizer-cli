use inquire::{InquireError, Select};

use crate::VIM_MODE;

pub fn choose_episode(episodes: Vec<String>) -> Result<String, ()> {
    clearscreen::clear().unwrap();

    let ans: Result<String, InquireError> =
        Select::new("Select the episode you want to watch:", episodes.clone())
            .without_help_message()
            .with_page_size(25)
            .with_vim_mode(unsafe { VIM_MODE })
            .prompt();

    match ans {
        Ok(choice) => Ok(choice),
        Err(_) => Err(println!("There was an error, please try again")),
    }
}
