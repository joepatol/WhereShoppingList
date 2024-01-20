mod data;
mod error;
mod config;
mod req;
mod interface;

pub use error::ScrapeError;
pub use interface::Scraper;
pub use req::get_html_document;
pub use config::{ConfigBuilder, ScrapeConfig};
pub use data::{ProductInfo, InDbProduct};