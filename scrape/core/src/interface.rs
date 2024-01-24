use super::{ProductInfo, RateLimiter};
use anyhow::Result;
use std::future::Future;

pub trait Scraper {
    fn scrape(&self, max_items: Option<usize>, rate_limiter: &RateLimiter) -> impl Future<Output = Result<Vec<ProductInfo>>> + Send;
}
