use scraper::{Html, ElementRef};
use scrape_core::ScrapeError;
use anyhow::Result;

pub fn get_nr_pages(document: &Html) -> u32 {
    let html_button_select = scraper::Selector::parse("button").unwrap();
    let html_page_bar_selector = scraper::Selector::parse("div.pagination").unwrap();
    let html_page_grid_selector = scraper::Selector::parse("div.pages-grid").unwrap();
    let mut html_page_bar = document.select(&html_page_bar_selector);
    let mut pages_grid = html_page_bar.next().unwrap().select(&html_page_grid_selector);
    
    let last: String = pages_grid
        .next()
        .unwrap()
        .select(&html_button_select)
        .last()
        .unwrap()
        .text()
        .collect();

    last.parse().unwrap()
}

pub fn get_name(html_product: &ElementRef) -> Result<String> {
    Ok(html_product
        .select(&scraper::Selector::parse("div.content").map_err(|_| ScrapeError::CSSSelectorFailed("Jumbo"))?)
        .next().ok_or(ScrapeError::InvalidStructureAssumed("Jumbo"))?
        .select(&scraper::Selector::parse("div.upper").map_err(|_| ScrapeError::CSSSelectorFailed("Jumbo"))?)
        .next().ok_or(ScrapeError::InvalidStructureAssumed("Jumbo"))?
        .select(&scraper::Selector::parse("div.name").map_err(|_| ScrapeError::CSSSelectorFailed("Jumbo"))?)
        .next().ok_or(ScrapeError::InvalidStructureAssumed("Jumbo"))?
        .select(&scraper::Selector::parse("h2").map_err(|_| ScrapeError::CSSSelectorFailed("Jumbo"))?)
        .next().ok_or(ScrapeError::InvalidStructureAssumed("Jumbo"))?
        .select(&scraper::Selector::parse("a").map_err(|_| ScrapeError::CSSSelectorFailed("Jumbo"))?)
        .map(|a| a.text().collect::<String>()).collect())
}

pub fn get_price(html_product: &ElementRef) -> Result<f32> {
    let html_price = html_product
    .select(&scraper::Selector::parse("div.jum-price").map_err(|_| ScrapeError::CSSSelectorFailed("Jumbo"))?)
    .next().ok_or(ScrapeError::InvalidStructureAssumed("Jumbo"))?
    .select(&scraper::Selector::parse("div.current-price").map_err(|_| ScrapeError::CSSSelectorFailed("Jumbo"))?)
    .next().ok_or(ScrapeError::InvalidStructureAssumed("Jumbo"))?;

    let mut whole_price: String = html_price
        .select(&scraper::Selector::parse("span.whole").map_err(|_| ScrapeError::CSSSelectorFailed("Jumbo"))?)
        .map(|a| a.text().collect::<String>()).collect();
    let frac_price: String = html_price
        .select(&scraper::Selector::parse("sup.fractional").map_err(|_| ScrapeError::CSSSelectorFailed("Jumbo"))?)
        .map(|a| a.text().collect::<String>()).collect();

    whole_price.push_str(".");
    whole_price.push_str(frac_price.as_str());

    Ok(whole_price.parse().map_err(|_| ScrapeError::FailedToParseStringValue("Jumbo"))?)
}