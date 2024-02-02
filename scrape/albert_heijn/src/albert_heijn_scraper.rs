use anyhow::{anyhow, Result};
use log::info;
use scrape_core::scrape_utils::build_selector;
use scrape_core::{ProductInfo, RateLimiter, Scraper, HtmlLoader, ResultCollector};
use super::parse::{get_product_name, get_price, get_links, get_product_url};

pub const BASE_URL: &str = "https://www.ah.nl";
const LETTER_URL: &str = "/producten/merk?letter=";
const LETTERS: [&str; 1] = ["l"];
// const LETTERS: [&str; 27] = [
//     "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "%23",
//     "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", 
// ];
pub const SRC: &str = "Albert Heijn";

pub struct AlbertHeijnScraper<'a, T: HtmlLoader + Send + Sync> {
    connector: &'a T
}

impl<'a, T: HtmlLoader + Send + Sync> AlbertHeijnScraper<'a, T> {
    pub fn new(connector: &'a T) -> Self {
        Self { connector }
    }

    pub async fn scrape_brand_urls_for_letter(&self, letter: &str) -> Result<Vec<String>> {
        let mut url = String::default();
        for sub_str in [BASE_URL, LETTER_URL, letter] {
            url.push_str(sub_str);
        }

        info!("Scraping brand urls at {}", &url);
        let document = self.connector.load(url).await?;

        Ok(get_links(document.root_element())?)
    }

    pub async fn scrape_brand_page(&self, url: &str) -> Result<Vec<ProductInfo>> {
        info!("Scraping brand url {}", url);
        let document = self.connector.load(url.to_owned()).await?;

        let product_container_selector = build_selector(
            "article", 
            SRC
        )?;
        let product_containers = document.select(&product_container_selector);

        let mut products = Vec::new();
        for product_container in product_containers.into_iter() {
            products.push(ProductInfo::new(
                get_product_name(product_container)?,
                get_price(product_container)?,
                get_product_url(product_container)?,
            ));
        };
        if products.len() == 0 {
            return Err(anyhow!("Found 0 products at {}", url));
        }
        Ok(products) 
    }
}

impl<'a, T: HtmlLoader + Send + Sync> Scraper for AlbertHeijnScraper<'a, T> {
    async fn scrape(&self, max_requests: Option<usize>, rate_limiter: &RateLimiter) ->  ResultCollector<ProductInfo> {
        info!(target: SRC, "Start scraping");
        let max_nr_requests: usize;

        match max_requests {
            Some(value) => max_nr_requests = value,
            None => max_nr_requests = usize::MAX,
        };
        info!("Limited number of requests to {}", &max_nr_requests);

        let mut link_futures = Vec::new();
        for letter in LETTERS {
            link_futures.push(self.scrape_brand_urls_for_letter(&letter));
        }

        let links: ResultCollector<String> = rate_limiter.run(link_futures).await.into_iter().flatten().collect();

        let mut futures = Vec::new();

        for link in links.iter_ok() {
            futures.push(self.scrape_brand_page(link));
            if futures.len() > max_nr_requests - LETTERS.len() {
                break
            };
        };

        let mut results: ResultCollector<ProductInfo> = rate_limiter.run(futures).await.into_iter().flatten().collect();
        results.inherit_errs(links);
        results
    }
}