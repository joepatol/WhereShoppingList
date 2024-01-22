mod funcs;

use funcs::scrape;
use scrape_core::connectors::get_html_document_from_url;
use scrape_core::ConfigBuilder;

const MAX_ITEMS_PER_SCRAPE: u32 = 48;

#[tokio::main]
async fn main() {
    let config = 
        ConfigBuilder::new()
        .scrape_max_items(MAX_ITEMS_PER_SCRAPE)
        .build();

    scrape(&config, get_html_document_from_url).await;
}
