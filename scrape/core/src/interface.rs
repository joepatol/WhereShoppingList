use crate::ResultCollector;
use super::{ProductInfo, RateLimiter};
use anyhow::Result;
use std::future::Future;

pub trait Scraper {
    fn scrape<R: RateLimiter + Send + Sync>(&self, max_requests: Option<usize>, rate_limiter: &R) -> impl Future<Output = ResultCollector<ProductInfo>> + Send;
}

pub trait HtmlLoader {
    fn load(&self, url: String) -> impl Future<Output = Result<scraper::Html>> + Send + Sync;
}