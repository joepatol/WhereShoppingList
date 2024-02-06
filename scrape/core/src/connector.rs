use super::{HtmlLoader, ScrapeError};
use anyhow::Result;
use rand::Rng;
use super::constants::USER_AGENTS;
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

async fn get_html_document_from_url(client: &reqwest::Client, url: String) -> Result<scraper::Html> {
    // Fetch a html document using a client
    let user_agent;
    {
        user_agent = USER_AGENTS[rand::thread_rng().gen_range(0..USER_AGENTS.len())];
    }
    let req = client
        .get(&url)
        .header(USER_AGENT, user_agent)
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