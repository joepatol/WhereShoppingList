mod data;
mod error;
mod config;
mod interface;
mod rate_limiter;
mod connector;
mod result_collector;
pub mod scrape_utils;
mod constants;

pub use reqwest::{Client as RequestClient, ClientBuilder as RequestClientBuilder, header as request_header};
pub use error::{ScrapeError, DbError};
pub use interface::{Scraper, HtmlLoader, AsyncExecutor};
pub use config::{ConfigBuilder, ScrapeConfig};
pub use data::{ProductInfo, InDbProduct, InDbError};
pub use rate_limiter::{SimpleRateLimiter, RandomDelayRateLimiter};
pub use connector::ReqwestHtmlLoader;
pub use result_collector::{ResultCollector, Transform, AsyncTransform};