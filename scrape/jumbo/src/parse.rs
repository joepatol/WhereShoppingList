use std::num::{ParseIntError, ParseFloatError};
use scraper::{Html, ElementRef};
use super::jumbo_scraper::SRC;
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

    let selectors = build_selectors(&selector_strings, SRC)?;
    let pages_grid = walk_selectors(
        document.root_element(), 
        &selectors, 
        SRC,
    )?;

    let btn_selector = build_selector("button", SRC)?;

    let last: String = pages_grid
        .select(&btn_selector)
        .last()
        .ok_or(ScrapeError::InvalidStructureAssumed{ src: SRC.to_string()})?
        .text()
        .collect();

    Ok(
        last.parse().map_err(|e: ParseIntError| {
            ScrapeError::FailedToParseStringValue{ src: SRC.to_string(), err: e.to_string() }
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

    let selectors = build_selectors(&selector_strings, SRC)?;
    let selected = walk_selectors(html_product, &selectors, SRC)?;

    let name_selector = build_selector("a", SRC)?;

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

    let selectors = build_selectors(&selector_strings, SRC)?;
    let html_price = walk_selectors(html_product, &selectors[..], "Jumbo")?;

    let whole_price_selector = build_selector("span.whole", SRC)?;
    let frac_price_selector = build_selector("sup.fractional", SRC)?;

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
            ScrapeError::FailedToParseStringValue{ src: SRC.to_string(), err: e.to_string() }
        })?
    )
}