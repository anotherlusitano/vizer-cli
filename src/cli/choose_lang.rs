use inquire::{InquireError, Select};

pub fn choose_lang(langs: Vec<String>) -> Result<String, ()> {
    clearscreen::clear().unwrap();

    let ans: Result<String, InquireError> = Select::new("Select the language option:", langs)
        .without_help_message()
        .with_page_size(2)
        .with_vim_mode(true)
        .prompt();

    match ans {
        Ok(choice) => Ok(choice),
        Err(_) => Err(println!("There was an error, please try again")),
    }
}
