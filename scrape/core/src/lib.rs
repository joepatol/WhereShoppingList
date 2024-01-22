mod data;
mod error;
mod config;
mod interface;
pub mod connectors;
pub mod scrape_utils;

pub use error::{ScrapeError, DbError};
pub use interface::Scraper;
pub use config::{ConfigBuilder, ScrapeConfig};
pub use data::{ProductInfo, InDbProduct};