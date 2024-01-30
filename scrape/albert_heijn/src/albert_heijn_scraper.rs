use std::future::Future;
use anyhow::Result;
use reqwest::dns::Resolve;
use scraper::{Element, Html, ElementRef};
use log::info;
use scrape_core::scrape_utils::{build_selector, walk_selectors};
use scrape_core::{ProductInfo, RateLimiter, ScrapeError, Scraper};

const BASE_URL: &str = "https://www.ah.nl/";
const LETTER_URL: &str = "producten?letter=";
const LETTERS: [&str; 27] = [
    "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "#",
];
const SRC: &str = "Albert Heijn";

pub fn src() -> String {
    SRC.to_string()
}

pub struct AlbertHeijnScraper<T> 
where
    T: Future<Output = Result<Html>> + Send,
{
    html_fetcher: fn(String) -> T
}

impl<T: Future<Output = Result<Html>> + Send> AlbertHeijnScraper<T> {
    pub fn new(fetch_func: fn(String) -> T) -> Self {
        Self { html_fetcher: fetch_func }
    }

    async fn get_links_for_letter(&self, letter: &str) -> Result<Vec<String>> {
        let mut url = String::default();
        for sub_str in [BASE_URL, LETTER_URL, letter] {
            url.push_str(sub_str);
        }
        let document = (self.html_fetcher)(url).await?;
        let source = src();

        let brand_links_selector = build_selector("div.brand-hub_links__E6cvr", &src())?;
        let brand_links_container = walk_selectors(document.root_element(), &[brand_links_selector], &source)?;
        let a_selector = build_selector("a", &src())?;
        let brand_links = brand_links_container.select(&a_selector);
        let links: Vec<String> = brand_links
            .into_iter()
            .map(|l| l
                .value()
                .attr("href")
                .ok_or(ScrapeError::InvalidStructureAssumed { src: src() }))
            .collect::<Result<Vec<&str>, ScrapeError>>()?
            .into_iter()
            .map(|e| e.to_string())
            .collect();
        Ok(links)
    }
}

impl<T: Future<Output = Result<Html>> + Send> Scraper for AlbertHeijnScraper<T> {
    async fn scrape(&self, max_items: Option<usize>, rate_limiter: &RateLimiter) -> Result<Vec<ProductInfo>> {
        let mut link_futures = Vec::new();
        for letter in LETTERS.iter() {
            link_futures.push(self.get_links_for_letter(&letter));
        }

        let links = rate_limiter.run(link_futures).await?;
        todo!()
    }
}