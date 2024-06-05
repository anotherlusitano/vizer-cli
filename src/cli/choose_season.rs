use selthi::Selthi;

use crate::{TRANSLATION, VIM_MODE};

pub fn choose_season(seasons: Vec<String>) -> Result<String, ()> {
    let language = TRANSLATION.get().unwrap();
    let vim_mode = VIM_MODE.get().unwrap();
    print!("\x1B[2J\x1B[1;1H");

    let help_msg = format!("{} {}", language.total_season_misc_text, seasons.len());

    let seasons = seasons.iter().map(String::as_str).collect();

    let ans = Selthi::new(language.select_season_misc_text, seasons)
        .with_help_message(&help_msg)
        .with_page_size(25)
        .with_vim_mode(*vim_mode)
        .prompt();

    match ans {
        Some(choice) => Ok(choice.to_string()),
        None => Err(println!("{}", language.choose_season_err)),
    }
}
