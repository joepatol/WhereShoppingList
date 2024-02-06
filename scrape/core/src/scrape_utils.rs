use scraper::{selector::ToCss, ElementRef, Selector};
use super::ScrapeError;
use anyhow::Result;

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
        .ok_or(ScrapeError::InvalidStructureAssumed{ src: src.to_string(), info: selector.to_css_string() })?;
    }
    Ok(element)
}