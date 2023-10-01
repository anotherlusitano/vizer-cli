use inquire::{InquireError, Select};

pub fn choose_season(seasons: Vec<String>) -> Result<String, ()> {
    clearscreen::clear().unwrap();

    let ans: Result<String, InquireError> =
        Select::new("Select the season you want to watch:", seasons.clone())
            .without_help_message()
            .with_page_size(25)
            .with_vim_mode(true)
            .prompt();

    match ans {
        Ok(choice) => Ok(choice),
        Err(_) => Err(println!("There was an error, please try again")),
    }
}
