mod funcs;

use funcs::scrape;
use scrape_core::connectors::get_html_document_from_url;
use scrape_core::{ConfigBuilder, RateLimiter};

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    let config = ConfigBuilder::new().build();
    let rate_limiter = RateLimiter::new(2000);
    scrape(
        &config, 
        get_html_document_from_url,
        &rate_limiter,
    ).await.unwrap();
}
