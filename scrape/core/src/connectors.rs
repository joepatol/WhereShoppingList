use scraper::Html;
use super::ScrapeError;
use anyhow::Result;

pub async fn get_html_document_from_url(url: String) -> Result<Html> {
    // Get an HTML document from a url
    let response = reqwest::get(&url).await;
    let html_content = response
        .map_err(|e| ScrapeError::FailedToConnect { url: url.clone(), err: e.to_string() })?
        .text()
        .await
        .map_err(|e| ScrapeError::FailedToParseHtml { url: url.clone(), err: e.to_string() })?;
    Ok(scraper::Html::parse_document(&html_content))
}