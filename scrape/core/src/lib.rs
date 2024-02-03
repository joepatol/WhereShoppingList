mod data;
mod error;
mod config;
mod interface;
mod rate_limiter;
mod connector;
mod result_collector;
pub mod scrape_utils;

pub use reqwest::{Client as RequestClient, ClientBuilder as RequestClientBuilder, header as request_header};
pub use error::{ScrapeError, DbError};
pub use interface::{Scraper, HtmlLoader};
pub use config::{ConfigBuilder, ScrapeConfig};
pub use data::{ProductInfo, InDbProduct, InDbError};
pub use rate_limiter::{RateLimiter, SimpleRateLimiter, RandomDelayRateLimiter};
pub use connector::ReqwestHtmlLoader;
pub use result_collector::{ResultCollector, Transform, AsyncTransform};