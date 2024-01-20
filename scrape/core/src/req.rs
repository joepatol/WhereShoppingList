use scraper::Html;

pub async fn get_html_document(url: String) -> Html {
    let response = reqwest::get(url).await;
    let html_content = response.unwrap().text().await.unwrap();
    scraper::Html::parse_document(&html_content)
}