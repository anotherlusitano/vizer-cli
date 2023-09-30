use inquire::{InquireError, Select};
use scraper::{Html, Selector};

use crate::media::Media;

#[tokio::main]
pub async fn search_media(media_name: &str) -> Media {
    let url = format!("https://vizer.in/pesquisar/{}", media_name);
    let response = reqwest::get(url).await.expect("Could not load url.");
    let html = response.text().await.unwrap();

    let document = Html::parse_document(html.as_str());

    let list_posters_selector = Selector::parse(r#"div[class="listItems"]"#).unwrap();
    let poster_selector = Selector::parse("a").unwrap();

    let list_media = document.select(&list_posters_selector).next().unwrap();

    let mut medias = Vec::new();

    for poster in list_media.select(&poster_selector) {
        let media_title = poster
            .value()
            .attr("title")
            .unwrap()
            .replace("Assistir ", "")
            .replace(" online", "");

        let media_link = poster.value().attr("href").unwrap();

        let media = Media {
            title: media_title,
            link: media_link.to_string(),
        };

        medias.push(media);
    }

    choose_media(medias).unwrap()
}

fn choose_media(medias: Vec<Media>) -> Result<Media, ()> {
    let options: Vec<String> = medias
        .iter()
        .enumerate()
        .map(|(index, item)| format!("{} {}", index + 1, item.title))
        .collect();

    let vec_str: Vec<&str> = options.iter().map(|s| s.as_str()).collect();

    clearscreen::clear().unwrap();
    let ans: Result<&str, InquireError> =
        Select::new("Select what you want to watch:", vec_str.clone())
            .without_help_message()
            .with_page_size(options.len())
            .with_vim_mode(true)
            .prompt();

    match ans {
        Ok(choice) => {
            let mut media_index = choice.split_whitespace();

            let index: usize = media_index.next().unwrap().parse::<usize>().unwrap();

            let media = medias[index - 1].clone();
            Ok(media)
        }
        Err(_) => Err(println!("There was an error, please try again")),
    }
}
