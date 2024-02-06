mod funcs;
mod response;
mod state;

use log::{info, LevelFilter};
use serde_json::json;
use simple_logger::SimpleLogger;
use warp::{Filter, Rejection, Reply, http::Response};
use funcs::scrape;
use scrape_core::ConfigBuilder;
use crate::state::StateKeeper;
use crate::response::{ScraperState, ScraperStateResponse};

const HOST: [u8; 4] = [0, 0, 0, 0];
const PORT: u16 = 7071;
type Result<T> = std::result::Result<T, Rejection>;

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    log::set_boxed_logger(Box::new(SimpleLogger::new()))
        .map(|()| log::set_max_level(LevelFilter::Info))
        .expect("Failed to initialize logger");

    let state_keeper = StateKeeper::default();
    let status_clone = state_keeper.clone();
    let func_clone = state_keeper.clone();

    let health_check_route = 
        warp::get()
        .and(warp::path("health_check"))
        .and_then(health_check);
    let scrape_route = 
        warp::post()
        .and(warp::path("scrape_func"))
        .map(move || func_clone.clone())
        .map(|state_clone| {
            tokio::spawn(async move {
                handler(state_clone).await
            });
            let scraper_state = ScraperStateResponse::new(ScraperState::Started);
            serde_json::to_string(&scraper_state).unwrap()
        });
    let status_route = 
        warp::get()
        .and(warp::path("status"))
        .map(move || status_clone.clone())
        .and_then(get_handler_state);

    let routes = 
        health_check_route
        .or(scrape_route)
        .or(status_route)
        .with(warp::cors()
        .allow_any_origin());

    warp::serve(routes).run((HOST, PORT)).await;
}

async fn health_check() -> Result<impl Reply> {
    Ok(Response::builder()
        .body(
            serde_json::to_string(&json!({"state": "ok"}))
            .unwrap()
        )
    )
}

async fn get_handler_state(state_keeper: StateKeeper<ScraperState>) -> Result<impl Reply>  {
    let scraper_state = state_keeper.get_state().await;
    let response = ScraperStateResponse::new(scraper_state);
    Ok(Response::builder().body(serde_json::to_string(&response).unwrap()))
}

async fn handler(state_keeper: StateKeeper<ScraperState>) {
    if state_keeper.get_state().await == ScraperState::Running {
        return
    }

    state_keeper.change_state(ScraperState::Running).await;
    let config = ConfigBuilder::new()
        .max_concurrent_requests(50)
        .build();

    match scrape(config).await {
        Ok(_) => { state_keeper.change_state(ScraperState::Success).await },
        Err(e) => { 
            info!("Scraping failed, message: {}", e);
            state_keeper.change_state(ScraperState::Failed).await 
        },
    };
}