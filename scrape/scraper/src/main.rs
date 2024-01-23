mod funcs;

use funcs::scrape;
use scrape_core::connectors::get_html_document_from_url;
use scrape_core::ConfigBuilder;

#[tokio::main(flavor = "multi_thread", worker_threads = 6)]
async fn main() {
    let config = ConfigBuilder::new().build();
    scrape(&config, get_html_document_from_url).await.unwrap();
}
