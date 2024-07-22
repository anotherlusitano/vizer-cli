use selthi::Select;

use crate::{TRANSLATION, VIM_MODE};

pub fn choose_season(seasons: Vec<&str>) -> Result<usize, ()> {
    let language = TRANSLATION.get().unwrap();
    let vim_mode = VIM_MODE.get().unwrap();
    print!("\x1B[2J\x1B[1;1H");

    let help_msg = format!("{} {}", language.total_season_misc_text, seasons.len());

    let ans = Select::new(language.select_season_misc_text, seasons)
        .with_help_message(&help_msg)
        .with_page_size(25)
        .with_vim_mode(*vim_mode)
        .prompt();

    match ans {
        Some(opt) => {
            let number = opt.split('º').next().unwrap();
            Ok(number.parse::<usize>().unwrap())
        }
        None => Err(println!("{}", language.choose_season_err)),
    }
}
