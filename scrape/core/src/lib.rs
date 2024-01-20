mod product_info;
mod error;
mod config;

use scraper::Html;
use reqwest;

pub use config::{ConfigBuilder, ScrapeConfig};
pub use product_info::ProductInfo;

pub async fn get_html_document(url: &str) -> Html {
    let response = reqwest::get(url).await;
    let html_content = response.unwrap().text().await.unwrap();
    scraper::Html::parse_document(&html_content)
}