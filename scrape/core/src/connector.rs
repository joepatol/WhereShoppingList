use super::HtmlLoader;
use anyhow::Result;
use super::scrape_utils::get_html_document_from_url;

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