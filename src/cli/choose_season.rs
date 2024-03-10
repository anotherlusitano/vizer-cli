use inquire::{InquireError, Select};

use crate::{TRANSLATION, VIM_MODE};

pub fn choose_season(seasons: Vec<String>) -> Result<String, ()> {
    print!("\x1B[2J\x1B[1;1H");

    let help_msg = format!("{} {}", TRANSLATION.total_season_misc_text, seasons.len());

    let ans: Result<String, InquireError> =
        Select::new(TRANSLATION.select_season_misc_text, seasons.clone())
            .with_help_message(&help_msg)
            .with_page_size(25)
            .with_vim_mode(unsafe { VIM_MODE })
            .prompt();

    match ans {
        Ok(choice) => Ok(choice),
        Err(_) => Err(println!("{}", TRANSLATION.choose_season_err)),
    }
}
