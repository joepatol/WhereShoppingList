use anyhow::Result;
use log::info;
use scrape_core::{HtmlLoader, ProductInfo, AsyncExecutor, ResultCollector, AsyncTransform, Scraper};
use scrape_core::scrape_utils::build_selector;
use super::parse::{get_name, get_price, get_nr_pages, get_product_url};

const PRODUCTS_PER_PAGE: usize = 24;
pub const BASE_URL: &str = "https://www.jumbo.com";
const URL: &str = "https://www.jumbo.com/producten";
const OFFSET_URL: &str = "/?offSet=";
pub const SRC: &str = "Jumbo";

pub struct JumboScraper<'a, T: HtmlLoader + Send + Sync> {
    connector: &'a T,
}

impl<'a, T: HtmlLoader + Send + Sync> JumboScraper<'a, T> {
    pub fn new(connector: &'a T) -> Self {
        Self { connector }
    }

    async fn scrape_page(&self, offset: String) -> Result<Vec<ProductInfo>> {
        let url = format!("{}{}{}", URL, OFFSET_URL, offset);
        info!("Scraping url {}", &url);
        
        let document = self.connector.load(url.clone()).await?;
        let selector = build_selector("article.product-container", SRC)?;
        let html_products = document.select(&selector);
    
        html_products
            .into_iter()
            .map(|html_product| -> Result<ProductInfo> {
                Ok(ProductInfo::new(
                get_name(html_product)?,
                get_price(html_product)?,
                get_product_url(html_product)?
                ))
            })
            .collect()
    }

    async fn scrape_nr_pages(&self) -> Result<usize> {
        let document = self.connector.load(URL.to_owned()).await?;
        Ok(get_nr_pages(&document)?)
    }
}

impl<'a, T: HtmlLoader + Send + Sync> Scraper for JumboScraper<'a, T> {
    async fn scrape<R: AsyncExecutor + Send + Sync>(&self, rate_limiter: &R) -> ResultCollector<ProductInfo> {
        info!(target: SRC, "Start scraping");

        let nr_pages = match self.scrape_nr_pages().await {
            Ok(amt) => amt,
            Err(e) => return ResultCollector::from(e),
        };
        info!("Found {} pages", &nr_pages);

        let offsets = (0..nr_pages)
            .map(|e| (e * PRODUCTS_PER_PAGE).to_string())
            .collect::<Vec<String>>();

        ResultCollector::from(offsets)
            .transform_async(|i| self.scrape_page(i), rate_limiter)
            .await
            .flatten()
    }
}