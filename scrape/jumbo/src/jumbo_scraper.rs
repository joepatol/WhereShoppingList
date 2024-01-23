use futures::future::join_all;
use scraper::Html;
use anyhow::Result;
use std::future::Future;
use scrape_core::{ScrapeConfig, Scraper, ProductInfo};
use scrape_core::scrape_utils::build_selector;
use super::parse::{get_name, get_price, get_nr_pages};

const PRODUCTS_PER_PAGE: u32 = 24;
const URL: &str = "https://www.jumbo.com/producten";
const OFFSET_URL: &str = "/?offSet=";
const SRC: &str = "Jumbo";

pub fn src() -> String {
    SRC.to_string()
}

pub struct JumboScraper<T: Future<Output = Result<Html>>> {
    pub html_fetcher: fn(String) -> T,
}

impl<T: Future<Output = Result<Html>>> JumboScraper<T> {
    pub fn new(fetch_func: fn(String) -> T) -> Self {
        Self { html_fetcher: fetch_func }
    }

    async fn scrape_page(&self, offset: String) -> Result<Vec<ProductInfo>> {
        let mut url = String::new();
        for slice in [URL, OFFSET_URL, &offset] {
            url.push_str(slice);
        }
        let document = (self.html_fetcher)(url).await?;
    
        let selector = build_selector("article.product-container", &src())?;
        let html_products = document.select(&selector);
        
        let mut products = Vec::new();
        for html_product in html_products.into_iter() {
            let product = ProductInfo::new(
                get_name(html_product)?,
                get_price(html_product)?,
            );
            products.push(product);
        };
        Ok(products)
    }
}

impl<T: Future<Output = Result<Html>>> Scraper for JumboScraper<T> {
    async fn scrape(&self, cfg: &ScrapeConfig) -> Result<Vec<ProductInfo>> {
        let max_nr_products: u32;
    
        match cfg.max_items {
            Some(value) => max_nr_products = value,
            None => {
                let document = (self.html_fetcher)(URL.to_string()).await?;
                let nr_pages = get_nr_pages(&document)?;
                max_nr_products = nr_pages * PRODUCTS_PER_PAGE
            },
        };
    
        let mut loaded_nr_products = 0;
        let mut futures = Vec::new();
    
        while loaded_nr_products < max_nr_products {
            futures.push(self.scrape_page(loaded_nr_products.to_string()));
            loaded_nr_products += PRODUCTS_PER_PAGE;
        };
        
        Ok(join_all(futures)
            .await
            .into_iter()
            .collect::<Result<Vec<Vec<ProductInfo>>>>()?
            .into_iter()
            .flatten()
            .collect()
        )
    }
}
