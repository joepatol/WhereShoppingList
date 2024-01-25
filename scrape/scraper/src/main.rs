mod funcs;
mod response;

use std::sync::Arc;
use log::{info, LevelFilter};
use simple_logger::SimpleLogger;
use funcs::scrape;
use scrape_core::connectors::get_html_document_from_url;
use scrape_core::ConfigBuilder;
use tokio::sync::Mutex;
use crate::response::{ScraperState, ScraperStateResponse};
use warp::{Filter, Rejection, Reply, http::Response};

const HOST: [u8; 4] = [0, 0, 0, 0];
const PORT: u16 = 7071;
type Result<T> = std::result::Result<T, Rejection>;

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    log::set_boxed_logger(Box::new(SimpleLogger::new()))
        .map(|()| log::set_max_level(LevelFilter::Info))
        .expect("Failed to initialize logger");

    let state: Arc<Mutex<ScraperState>> = Arc::new(Mutex::new(ScraperState::Idle));
    let status_clone = state.clone();
    let func_clone = state.clone();

    let health_check_route = warp::get()
        .and(warp::path("health_check"))
        .and_then(health_check);
    let scrape_route = warp::get()
        .and(warp::path("scrape_func"))
        .map(move || func_clone.clone())
        .map(|state_clone| {
            tokio::spawn(async move {
                handler(state_clone).await
            });
            let scarper_state = ScraperStateResponse::new(ScraperState::Started);
            serde_json::to_string(&scarper_state).unwrap()
        });
    let status_route = warp::get()
        .and(warp::path("status"))
        .map(move || status_clone.clone())
        .and_then({
            get_handler_state
        });

    let routes = health_check_route
        .or(scrape_route)
        .or(status_route)
        .with(warp::cors()
        .allow_any_origin());

    warp::serve(routes).run((HOST, PORT)).await;
}

async fn health_check() -> Result<impl Reply> {
    Ok(Response::builder().body("Hi, all is looking dandy!"))
}

async fn get_handler_state(state: Arc<Mutex<ScraperState>>) -> Result<impl Reply>  {
    let scraper_state = get_scraper_state(state).await;
    let response = ScraperStateResponse::new(scraper_state);
    Ok(Response::builder().body(serde_json::to_string(&response).unwrap()))
}

async fn handler(mut state: Arc<Mutex<ScraperState>>) {
    if get_scraper_state(state.clone()).await == ScraperState::Running {
        return
    }

    state = change_scraper_state(state, ScraperState::Running).await;
    let config = ConfigBuilder::new()
        .max_concurrent_requests(50)
        .build();

    match scrape(config, get_html_document_from_url).await {
        Ok(_) => { change_scraper_state(state, ScraperState::Success).await },
        Err(e) => { 
            info!("Scraping failed, message: {}", e);
            change_scraper_state(state, ScraperState::Failed).await 
        },
    };
}

async fn get_scraper_state(state: Arc<Mutex<ScraperState>>) -> ScraperState {
    let istate = state.lock().await; 
    let cur_state = istate.clone();
    drop(istate); 
    cur_state
}

async fn change_scraper_state(state: Arc<Mutex<ScraperState>>, new_state: ScraperState) -> Arc<Mutex<ScraperState>>{
    let mut istate = state.lock().await; 
    *istate = new_state;
    drop(istate); 
    state
}