use std::num::{ParseIntError, ParseFloatError};
use scraper::{Html, ElementRef};
use super::jumbo_scraper::src;
use anyhow::Result;
use scrape_core::ScrapeError;
use scrape_core::scrape_utils::{
    walk_selectors,
    build_selector,
    build_selectors,
};

pub fn get_nr_pages(document: &Html) -> Result<usize> {
    let selector_strings = [
        "div.pagination",
        "div.pages-grid",
    ]; 

    let source: String = src();

    let selectors = build_selectors(&selector_strings, &src())?;
    let pages_grid = walk_selectors(
        document.root_element(), 
        &selectors, 
        &source,
    )?;

    let btn_selector = build_selector("button", &source)?;

    let last: String = pages_grid
        .select(&btn_selector)
        .last()
        .ok_or(ScrapeError::InvalidStructureAssumed{ src: src()})?
        .text()
        .collect();

    Ok(
        last.parse().map_err(|e: ParseIntError| {
            ScrapeError::FailedToParseStringValue{ src: src(), err: e.to_string() }
        })?
    )
}

pub fn get_name(html_product: ElementRef) -> Result<String> {
    let selector_strings = [
        "div.content",
        "div.upper",
        "div.name",
        "h2",
    ];

    let source = src();

    let selectors = build_selectors(&selector_strings, &src())?;
    let selected = walk_selectors(html_product, &selectors, &source)?;

    let name_selector = build_selector("a", &source)?;

    Ok(selected
        .select(&name_selector)
        .map(|a| a.text().collect::<String>()).collect()
    )
}

pub fn get_price(html_product: ElementRef) -> Result<f32> {
    let selector_strings = [
        "div.jum-price",
        "div.current-price",
    ];

    let selectors = build_selectors(&selector_strings, &src())?;
    let html_price = walk_selectors(html_product, &selectors[..], "Jumbo")?;

    let whole_price_selector = build_selector("span.whole", &src())?;
    let frac_price_selector = build_selector("sup.fractional", &src())?;

    let mut whole_price: String = html_price
        .select(&whole_price_selector)
        .map(|a| a.text().collect::<String>()).collect();
    let frac_price: String = html_price
        .select(&frac_price_selector)
        .map(|a| a.text().collect::<String>()).collect();

    whole_price.push_str(".");
    whole_price.push_str(frac_price.as_str());

    Ok(
        whole_price.parse().map_err(|e: ParseFloatError| {
            ScrapeError::FailedToParseStringValue{ src: src(), err: e.to_string() }
        })?
    )
}