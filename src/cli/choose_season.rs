use inquire::{InquireError, Select};

use crate::VIM_MODE;

pub fn choose_season(seasons: Vec<String>) -> Result<String, ()> {
    print!("\x1B[2J\x1B[1;1H");

    let help_msg = format!("Total of seasons to watch: {}", seasons.len());

    let ans: Result<String, InquireError> =
        Select::new("Select the season you want to watch:", seasons.clone())
            .with_help_message(&help_msg)
            .with_page_size(25)
            .with_vim_mode(unsafe { VIM_MODE })
            .prompt();

    match ans {
        Ok(choice) => Ok(choice),
        Err(_) => Err(println!("There was an error, please try again")),
    }
}
