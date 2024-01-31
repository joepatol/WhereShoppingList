use anyhow::Result;
use log::info;
use scrape_core::{Scraper, ProductInfo, RateLimiter, HtmlLoader};
use scrape_core::scrape_utils::build_selector;
use super::parse::{get_name, get_price, get_nr_pages};

const PRODUCTS_PER_PAGE: usize = 24;
const URL: &str = "https://www.jumbo.com/producten";
const OFFSET_URL: &str = "/?offSet=";
pub const SRC: &str = "Jumbo";

#[derive(Clone)]
pub struct JumboScraper<'a ,T: HtmlLoader + Send + Sync> {
    connector: &'a T,
}

impl<'a, T: HtmlLoader + Send + Sync> JumboScraper<'a, T> {
    pub fn new(connector: &'a T) -> Self {
        Self { connector }
    }

    async fn scrape_page(&self, offset: String) -> Result<Vec<ProductInfo>> {
        let mut url = String::new();
        for slice in [URL, OFFSET_URL, &offset] {
            url.push_str(slice);
        }
        let mut products = Vec::new();
        
        let document = self.connector.load(url.clone()).await?;
        let selector = build_selector("article.product-container", SRC)?;
        let html_products = document.select(&selector);
    
        for html_product in html_products.into_iter() {
            let product = ProductInfo::new(
                get_name(html_product)?,
                get_price(html_product)?,
            );
            products.push(product);
        };
        info!(target: SRC, "Scraped url {}", url);
        Ok(products)
    }
}

impl<'a, T: HtmlLoader + Send + Sync> Scraper for JumboScraper<'a, T> {
    async fn scrape(&self, max_requests: Option<usize>, rate_limiter: &RateLimiter) -> Result<Vec<ProductInfo>> {
        info!(target: SRC, "Start scraping");
        let max_nr_requests: usize;
    
        match max_requests {
            Some(value) => max_nr_requests = value,
            None => max_nr_requests = usize::MAX,
        };
        info!("Limited number of requests to {}", &max_nr_requests);

        // scraper::Html is not Send, so get it in it's own scope so we don't carry it
        // across an await point
        let total_products: usize;
        {
            let document = self.connector.load(URL.to_owned()).await?;
            let nr_pages: usize = get_nr_pages(&document)?;
            total_products = nr_pages * PRODUCTS_PER_PAGE;
        }
        
        let mut loaded_nr_products = 0;
        let mut futures = Vec::new();

        while loaded_nr_products < total_products {
            futures.push(self.scrape_page(loaded_nr_products.to_string()));
            if futures.len() == max_nr_requests - 1 {
                break
            };
            loaded_nr_products += PRODUCTS_PER_PAGE;
        };
        Ok(rate_limiter.run(futures).await?)
    }
}