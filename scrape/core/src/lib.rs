mod data;
mod error;
mod config;
mod interface;
mod rate_limiter;
pub mod connectors;
pub mod scrape_utils;

pub use reqwest::{Client as RequestClient, ClientBuilder as RequestClientBuilder, header as request_header};
pub use error::{ScrapeError, DbError};
pub use interface::Scraper;
pub use config::{ConfigBuilder, ScrapeConfig};
pub use data::{ProductInfo, InDbProduct};
pub use rate_limiter::RateLimiter;