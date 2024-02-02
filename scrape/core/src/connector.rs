use super::{HtmlLoader, ScrapeError};
use anyhow::Result;
use rand::Rng;
use reqwest::header::{USER_AGENT, ACCEPT_LANGUAGE, REFERER, ACCEPT_ENCODING};

pub struct ReqwestHtmlLoader<'a> {
    client: &'a reqwest::Client,
}

impl<'a> ReqwestHtmlLoader<'a> {
    pub fn new(client: &'a reqwest::Client) -> Self {
        Self { client }
    }
}

impl<'a> HtmlLoader for ReqwestHtmlLoader<'a> {
    async fn load(&self, url: String) -> Result<scraper::Html> {
        Ok(get_html_document_from_url(self.client, url).await?)
    }
}

const USER_AGENTS: [&str; 4] = [
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Mobile Safari/537.36",
    "Mozilla/5.0 (Linux; Android 13; SM-S901U) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Mobile Safari/537.36",
    "Mozilla/5.0 (Linux; Android 13; Pixel 6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Mobile Safari/537.36",
];

async fn get_html_document_from_url(client: &reqwest::Client, url: String) -> Result<scraper::Html> {
    // Fetch a html document using a client
    let idx = rand::thread_rng().gen_range(0..4);
    let req = client
        .get(&url)
        .header(USER_AGENT, USER_AGENTS[idx])
        .header(ACCEPT_LANGUAGE, "en-US,en;q=0.5" )
        .header(REFERER, "https://google.com/")
        .header(ACCEPT_ENCODING, "gzip, deflate, br");

    let response = 
        req
        .send()
        .await;

    let html_content = response
        .map_err(|e| ScrapeError::FailedToConnect { url: url.clone(), err: e.to_string() })?
        .text()
        .await
        .map_err(|e| ScrapeError::FailedToParseHtml { url: url.clone(), err: e.to_string() })?;

    Ok(scraper::Html::parse_document(&html_content))
}