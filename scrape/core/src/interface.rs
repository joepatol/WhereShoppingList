use crate::ResultCollector;
use super::ProductInfo;
use anyhow::Result;
use std::future::Future;

pub trait AsyncExecutor {
    fn run<T: Send + Sync>(&self, futures: Vec<impl Future<Output = T> + Send + Sync>) -> impl Future<Output = Vec<Result<T>>> + Send + Sync;
}

pub trait Scraper {
    fn scrape<R: AsyncExecutor + Send + Sync>(&self, rate_limiter: &R) -> impl Future<Output = ResultCollector<ProductInfo>> + Send;
}

pub trait HtmlLoader {
    fn load(&self, url: String) -> impl Future<Output = Result<scraper::Html>> + Send + Sync;
}