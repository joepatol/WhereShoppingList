use super::{ScrapeConfig, ProductInfo};
use anyhow::Result;
use std::future::Future;

pub trait Scraper {
    fn scrape(&self, cfg: &ScrapeConfig) -> impl Future<Output = Result<Vec<ProductInfo>>>;
}
