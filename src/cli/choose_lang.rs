use inquire::{InquireError, Select};

use crate::{TRANSLATION, VIM_MODE};

pub fn choose_lang(langs: Vec<String>) -> Result<String, ()> {
    let language = TRANSLATION.get().unwrap();
    print!("\x1B[2J\x1B[1;1H");

    let ans: Result<String, InquireError> = Select::new(language.select_lang_misc_text, langs)
        .without_help_message()
        .with_page_size(2)
        .with_vim_mode(unsafe { VIM_MODE })
        .prompt();

    match ans {
        Ok(choice) => Ok(choice),
        Err(_) => Err(println!("{}", language.choose_lang_err)),
    }
}
