use scraper::{Html, Selector};

use crate::{media::Media, TRANSLATION};

pub async fn get_medias(media_name: &str) -> Vec<Media> {
    let language = TRANSLATION.get().unwrap();

    let url = format!("https://vizer.in/pesquisar/{}", media_name);
    let response = reqwest::get(url).await.expect(language.response_expect);
    let html = response.text().await.unwrap();

    let document = Html::parse_document(html.as_str());

    let list_posters_selector = Selector::parse(r#"div[class="listItems"]"#).unwrap();
    let img_selector = Selector::parse(r#"img[class="img"]"#).unwrap();
    let poster_selector = Selector::parse("a").unwrap();

    let list_media = document.select(&list_posters_selector).next().unwrap();

    let mut medias = Vec::new();

    for (poster, img) in list_media
        .select(&poster_selector)
        .zip(list_media.select(&img_selector))
    {
        let media_title = poster
            .value()
            .attr("title")
            .unwrap()
            .replace("Assistir ", "")
            .replace(" online", "");

        let media_url = poster.value().attr("href").unwrap();

        let img_src = img.value().attr("src").unwrap();

        // we replace the "size" of the image in the url
        // to improve the quality of the image
        let img_url = format!("https://vizertv.in{}", img_src.replace("t/185", "t/342"));

        let media = Media {
            title: media_title,
            url: media_url.to_string(),
            poster_url: img_url,
        };

        medias.push(media);
    }

    medias
}
