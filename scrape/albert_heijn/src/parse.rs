use scraper::ElementRef;
use std::num::ParseFloatError;
use anyhow::Result;
use scrape_core::ScrapeError;
use scrape_core::scrape_utils::{build_selector, walk_selectors};
use super::albert_heijn_scraper::{SRC, BASE_URL};

pub fn get_product_url(element: ElementRef) -> Result<String> {
    let selector = build_selector("a", SRC)?;
    let link_html = walk_selectors(element, &[selector], SRC)?;
    let link = link_html
        .attr("href")
        .ok_or(ScrapeError::InvalidStructureAssumed { src: SRC.to_string() })?;
    let mut prod_url = BASE_URL.to_owned();
    prod_url.push_str(link);
    Ok(prod_url)
}

pub fn get_links(element: ElementRef) -> Result<Vec<String>> {
    let brand_links_selector = build_selector("div.brand-hub_links__E6cvr", SRC)?;
    let brand_links_container = walk_selectors(element, &[brand_links_selector], SRC)?;
    let a_selector = build_selector("a", SRC)?;
    let brand_links = brand_links_container.select(&a_selector);
    Ok(
        brand_links
        .into_iter()
        .map(|l| l
            .value()
            .attr("href")
            .ok_or(ScrapeError::InvalidStructureAssumed { src: SRC.to_string() }))
        .collect::<Result<Vec<&str>, ScrapeError>>()?
        .into_iter()
        .map(|e| {
            let mut url = BASE_URL.to_string();
            url.push_str(e);
            url
        })
        .collect()
    )
}

pub fn get_product_name(element: ElementRef) -> Result<String> {
    let info_selector = build_selector("a", SRC)?;
    let product_name = walk_selectors(element, &[info_selector], SRC)?
        .attr("title")
        .ok_or(ScrapeError::InvalidStructureAssumed { src: SRC.to_string() })?;
    Ok(product_name.to_owned())
}

pub fn get_price(element: ElementRef) -> Result<f32> {
    let price_selector = build_selector("div.price-amount_root__Sa88q", SRC)?;
    let price_str: String = element
        .select(&price_selector)
        .last()
        .ok_or(ScrapeError::InvalidStructureAssumed { src: SRC.to_string() })?
        .text()
        .collect::<String>();

    Ok(price_str.parse().map_err(|e: ParseFloatError| {
        ScrapeError::FailedToParseStringValue{ src: SRC.to_string(), err: e.to_string() }
    })?)
}