use std::collections::HashMap;
use scraper::Html;
use log::info;
use std::future::Future;
use jumbo::JumboScraper;
use scrape_core::{RateLimiter, Scraper};
use sql::{tables, self};
use anyhow::Result;
use scrape_core::{InDbProduct, ScrapeConfig};

pub async fn scrape<T: Future<Output = Result<Html>> + Send>(config: ScrapeConfig, connector_func: fn(String) -> T) -> Result<()> {
    info!("Starting scrape...");
    info!("Setting up SqlPool connection");
    let pool = sql::connect().await?;
    info!("Using configuration: {:?}", &config);
    info!("Clearing tables");
    tables::products::truncate(&pool).await?;
    info!("Assembling scrapers...");
    let scrapers = build_scrapers(connector_func);
    info!("Scraping...");
    let rate_limiter = RateLimiter::new(config.max_concurrent_requests);
    let db_products = run_scrapers(&config, scrapers, &rate_limiter).await?;
    info!("Writing new scrapes to db...");
    tables::products::insert(&db_products, &pool).await?;
    info!("All done");
    pool.close().await;
    Ok(())
}

fn build_scrapers<T: Future<Output = Result<Html>> + Send>(connector_func: fn(String) -> T) -> HashMap<&'static str, impl Scraper> {
    HashMap::from([
        ("Jumbo", JumboScraper::new(connector_func))
    ])
}

async fn run_scrapers(
    cfg: &ScrapeConfig, 
    scrapers: HashMap<&'static str, impl Scraper>,
    rate_limiter: &RateLimiter,
) -> Result<Vec<InDbProduct>> {
    let mut db_products: Vec<InDbProduct> = Vec::new();
    for (scraper_name, scraper) in scrapers.iter() {
        let products = scraper.scrape(cfg.max_items, rate_limiter).await?;
        let in_db_products = 
            products
            .into_iter()
            .map(|p| 
                InDbProduct::new(scraper_name.to_string(), p)
            );
        
        db_products.extend(in_db_products);
    }
    Ok(db_products)
}