use selthi::Input;

use crate::TRANSLATION;

pub fn get_media_name_from_user() -> Result<String, ()> {
    let language = TRANSLATION.get().unwrap();

    let ans: Option<String> = Input::new(language.select_media_misc_text)
        .with_minimum_chars(4)
        .prompt();

    match ans {
        Some(media_name) => Ok(media_name),
        None => Err(println!("{}", language.choose_media_err)),
    }
}
