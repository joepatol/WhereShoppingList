mod data;
mod error;
mod config;
mod interface;
mod rate_limiter;
mod connector;
pub mod scrape_utils;

pub use reqwest::{Client as RequestClient, ClientBuilder as RequestClientBuilder, header as request_header};
pub use error::{ScrapeError, DbError};
pub use interface::{Scraper, HtmlLoader};
pub use config::{ConfigBuilder, ScrapeConfig};
pub use data::{ProductInfo, InDbProduct};
pub use rate_limiter::RateLimiter;
pub use connector::ReqwestHtmlLoader;