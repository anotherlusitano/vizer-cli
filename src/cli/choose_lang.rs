use selthi::Selthi;

use crate::{TRANSLATION, VIM_MODE};

pub fn choose_lang(langs: Vec<String>) -> Result<String, ()> {
    let language = TRANSLATION.get().unwrap();
    let vim_mode = VIM_MODE.get().unwrap();
    print!("\x1B[2J\x1B[1;1H");

    let langs = langs.iter().map(String::as_str).collect();

    let ans = Selthi::new(language.select_lang_misc_text, langs)
        .without_help_message()
        .with_page_size(2)
        .with_vim_mode(*vim_mode)
        .prompt();

    match ans {
        Some(choice) => Ok(choice.to_string()),
        None => Err(println!("{}", language.choose_lang_err)),
    }
}
