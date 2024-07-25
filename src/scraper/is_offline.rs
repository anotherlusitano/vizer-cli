use scraper::{Html, Selector};

use crate::TRANSLATION;

pub async fn is_offline() -> bool {
    let language = TRANSLATION.get().unwrap();

    let url = "https://vizer.in";
    let response = reqwest::get(url).await.expect(language.response_expect);
    let html = response.text().await.unwrap();

    let document = Html::parse_document(html.as_str());
    let selector = Selector::parse("body").unwrap();

    let body = document.select(&selector).next().unwrap();
    let warning = body.text().next().unwrap();

    warning == "error code: 523"
}
