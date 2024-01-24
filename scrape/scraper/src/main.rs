mod funcs;

use std::net::Ipv4Addr;

use funcs::scrape;
use scrape_core::connectors::get_html_document_from_url;
use scrape_core::ConfigBuilder;
use warp::{Filter, Rejection, Reply, http::Response};

type Result<T> = std::result::Result<T, Rejection>;

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    let route = warp::get()
        // Warp path needs to match Azure functions URL
        .and(warp::path("api"))
        .and(warp::path("scrapeFunc"))
        .and_then(handler);

    warp::serve(route).run((Ipv4Addr::LOCALHOST, 7071)).await;
}

async fn handler() -> Result<impl Reply> {
    let config = ConfigBuilder::new().max_concurrent_requests(100).build();
    match scrape(&config, get_html_document_from_url).await {
        Ok(_) => Ok(Response::builder().body(String::from("Scrape successful"))),
        Err(e) => Ok(Response::builder().body(format!("Scrape failed, message: {}", e)))
    }
}