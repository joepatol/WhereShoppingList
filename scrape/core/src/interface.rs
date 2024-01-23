use super::{ScrapeConfig, ProductInfo, RateLimiter};
use anyhow::Result;
use std::future::Future;

pub trait Scraper {
    fn scrape(&self, cfg: &ScrapeConfig, rate_limiter: &RateLimiter) -> impl Future<Output = Result<Vec<ProductInfo>>>;
}
