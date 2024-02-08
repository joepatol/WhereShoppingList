use anyhow::Result;
use log::info;
use scrape_core::scrape_utils::build_selector;
use scrape_core::{AsyncTransform, HtmlLoader, ProductInfo, AsyncExecutor, ResultCollector, ScrapeError, Scraper};
use super::parse::{get_product_name, get_price, get_links, get_product_url};

pub const SRC: &str = "Albert Heijn";
pub const BASE_URL: &str = "https://www.ah.nl";
pub const PAGE_PART: &str = "?page=";
pub const OFFSET_PART: &str = "&withOffset=true";
const LETTER_URL: &str = "/producten/merk?letter=";
const PRODUCTS_PER_PAGE: usize = 36;
const LETTERS: [&str; 27] = [
    "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o",
    "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "%23",
];

pub struct AlbertHeijnScraper<'a, T: HtmlLoader + Send + Sync> {
    connector: &'a T,
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

    async fn scrape_product_link_until_exhausted(&self, url: String) -> ResultCollector<ProductInfo> {
        let mut should_break = false;
        let mut offset = 0;
        let mut collector = ResultCollector::new();

        loop {
            let products_result = self.scrape_page_with_offset(&url, offset).await;
            
            match &products_result {
                Ok(prods) => {
                    if prods.len() < PRODUCTS_PER_PAGE {
                        should_break = true;
                    }
                },
                Err(_) => should_break = true,
            };

            collector.collect(products_result);            
            if should_break {
                break;
            }
            offset += 1;
        }
        collector.flatten()
    }

    async fn scrape_page_with_offset(&self, url: &str, offset: usize) -> Result<Vec<ProductInfo>> {
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
    async fn scrape<R: AsyncExecutor + Send + Sync>(&self, rate_limiter: &R) ->  ResultCollector<ProductInfo> {
        info!(target: SRC, "Start scraping");
        ResultCollector::from(LETTERS.to_vec())
            .transform_async(|l| self.scrape_brand_urls_for_letter(l), rate_limiter)
            .await
            .flatten()
            .transform_async(|url| self.scrape_product_link_until_exhausted(url), rate_limiter)
            .await
    }
}