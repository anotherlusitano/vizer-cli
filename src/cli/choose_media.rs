use inquire::{InquireError, Select};

use crate::{media::Media, TRANSLATION, VIM_MODE};

pub fn choose_media(medias: Vec<Media>) -> Result<Media, ()> {
    let options: Vec<String> = medias
        .iter()
        .enumerate()
        .map(|(index, item)| format!("{} {}", index + 1, item.title))
        .collect();

    let vec_str: Vec<&str> = options.iter().map(|s| s.as_str()).collect();

    let help_msg = format!("{} {}", TRANSLATION.total_media_misc_text, vec_str.len());

    print!("\x1B[2J\x1B[1;1H");
    let ans: Result<&str, InquireError> =
        Select::new(TRANSLATION.select_media_misc_text, vec_str.clone())
            .with_help_message(&help_msg)
            .with_page_size(25)
            .with_vim_mode(unsafe { VIM_MODE })
            .prompt();

    match ans {
        Ok(choice) => {
            let mut media_index = choice.split_whitespace();

            let index: usize = media_index.next().unwrap().parse::<usize>().unwrap();

            let media = medias[index - 1].clone();
            Ok(media)
        }
        Err(_) => Err(println!("{}", TRANSLATION.choose_media_err)),
    }
}
