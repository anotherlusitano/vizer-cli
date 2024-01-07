use scraper::{Html, Selector};

use crate::media::Media;

#[tokio::main]
pub async fn get_medias(media_name: &str) -> Vec<Media> {
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

    medias
}
