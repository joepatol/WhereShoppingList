use scraper::{ElementRef, Selector, Html};
use rand::Rng;
use super::ScrapeError;
use anyhow::Result;
use reqwest::header::{USER_AGENT, ACCEPT_LANGUAGE, REFERER, ACCEPT_ENCODING};

const USER_AGENTS: [&str; 5] = [
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Mobile Safari/537.36",
    "Mozilla/5.0 (Linux; Android 13; SM-S901U) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Mobile Safari/537.36",
    "Mozilla/5.0 (Linux; Android 13; Pixel 6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Mobile Safari/537.36",
    "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)",
];

pub async fn get_html_document_from_url(client: &reqwest::Client, url: String) -> Result<Html> {
    // Fetch a html document using a client
    let num = rand::thread_rng().gen_range(0..5);
    let req = client
        .get(&url)
        .header(USER_AGENT, USER_AGENTS[num])
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

pub fn build_selector<'a>(selector_string: &'a str, src: &'a str) -> Result<Selector> {
    // Build a scraper::Selector from a CSS selector (e.g. "div.classname")
    Ok(
        scraper::Selector::parse(selector_string)
        .map_err(|e| ScrapeError::CSSSelectorFailed{ src: src.to_string(), err: e.to_string()})?
    )
}

pub fn build_selectors(selector_strings: &[&str], src: &str) -> Result<Vec<Selector>> {
    // Build multiple scraper::Selector instances from a slice of CSS selectors
    Ok(selector_strings.iter().map(|s| {
        build_selector(s, src)
    }).collect::<Result<Vec<Selector>>>()?)
}

pub fn walk_selectors<'a>(mut element: ElementRef<'a>, selectors: &[Selector], src: &'a str) -> Result<ElementRef<'a>> {
    // Take a HTML element and a slice of selectors, applies each selector and takes the first found element
    // then the next selector is applied
    for selector in selectors.iter() {
        element = element
        .select(&selector)
        .next()
        .ok_or(ScrapeError::InvalidStructureAssumed{ src: src.to_string() })?;
    }
    Ok(element)
}