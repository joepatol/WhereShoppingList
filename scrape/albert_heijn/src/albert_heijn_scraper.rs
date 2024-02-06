use anyhow::Result;
use log::info;
use scrape_core::scrape_utils::build_selector;
use scrape_core::{AsyncTransform, HtmlLoader, ProductInfo, RateLimiter, ResultCollector, ScrapeError, Scraper};
use super::parse::{get_product_name, get_price, get_links, get_product_url};

pub const BASE_URL: &str = "https://www.ah.nl";
pub const PAGE_PART: &str = "?page=";
pub const OFFSET_PART: &str = "&withOffset=true";
const LETTER_URL: &str = "/producten/merk?letter=";
const LETTERS: [&str; 27] = [
    "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "%23",
    "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", 
];
pub const SRC: &str = "Albert Heijn";

pub struct AlbertHeijnScraper<'a, T: HtmlLoader + Send + Sync> {
    connector: &'a T
}

impl<'a, T: HtmlLoader + Send + Sync> AlbertHeijnScraper<'a, T> {
    pub fn new(connector: &'a T) -> Self {
        Self { connector }
    }

    async fn scrape_brand_urls_for_letter(&self, letter: &str) -> Result<Vec<String>> {
        let url = format!("{}{}{}", BASE_URL, LETTER_URL, letter);

        info!("Scraping brand urls at {}", &url);
        let document = self.connector.load(url).await?;

        Ok(get_links(document.root_element())?)
    }

    async fn scrape_page_with_offset(&self, url: String, offset: usize) -> Result<Vec<ProductInfo>> {
        let offset_url = format!("{}{}{}{}", url, PAGE_PART, offset.to_string(), OFFSET_PART);
        info!("Scraping url {}", &offset_url);
        let document = self.connector.load(offset_url.clone()).await?;
        let product_container_selector = build_selector(
            "article", 
            SRC
        )?;
        let product_containers = document.select(&product_container_selector);

        let result: Result<Vec<ProductInfo>> = product_containers
            .into_iter()
            .map(|product_container| -> Result<ProductInfo> {
                Ok(ProductInfo::new(
                    get_product_name(product_container)?,
                    get_price(product_container)?,
                    get_product_url(product_container)?,
                ))
            })
            .collect();

        match result {
            Ok(products) => {
                if products.len() == 0 {
                    return Err(ScrapeError::NoProductsFound { src: SRC.to_owned(), url: offset_url }.into())
                };
                Ok(products)
            },
            Err(e) => Err(e)
        }
    }
}

impl<'a, T: HtmlLoader + Send + Sync> Scraper for AlbertHeijnScraper<'a, T> {
    async fn scrape<R: RateLimiter + Send + Sync>(&self, max_requests: Option<usize>, rate_limiter: &R) ->  ResultCollector<ProductInfo> {
        info!(target: SRC, "Start scraping");
        let max_nr_requests: usize;

        match max_requests {
            Some(value) => max_nr_requests = value,
            None => max_nr_requests = usize::MAX,
        };
        info!("Limited number of requests to {}", &max_nr_requests);

        let mut iterator = (0..10).collect::<Vec<usize>>().into_iter();
        if iterator.len() * LETTERS.len() > max_nr_requests - LETTERS.len() {
            let len_limit = (max_nr_requests - LETTERS.len()) / LETTERS.len();
            iterator = iterator.take(len_limit).collect::<Vec<usize>>().into_iter();
        }

        ResultCollector::from(LETTERS.to_vec())
            .transform_async(|l| self.scrape_brand_urls_for_letter(l), rate_limiter)
            .await
            .flatten()
            .explode(&iterator)
            .transform_async(|inp| self.scrape_page_with_offset(inp.0, inp.1), rate_limiter)
            .await
            .flatten()
    }
}