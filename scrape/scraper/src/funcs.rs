use scraper::Html;
use log::info;
use std::future::Future;
use jumbo::JumboScraper;
use albert_heijn::AlbertHeijnScraper;
use scrape_core::{RateLimiter, Scraper};
use sql::{tables, self, PgPool};
use anyhow::Result;
use scrape_core::{InDbProduct, ScrapeConfig, RequestClient};

pub async fn scrape<T: Future<Output = Result<Html>> + Send>(config: ScrapeConfig, connector_func: fn(RequestClient, String) -> T) -> Result<()> {
    info!("Starting scrape...");
    info!("Setting up SqlPool connection");
    let pool = sql::connect().await?;
    info!("Using configuration: {:?}", &config);
    info!("Clearing tables");
    tables::products::truncate(&pool).await?;
    info!("Assembling scrapers...");
    info!("Scraping...");
    let rate_limiter = RateLimiter::new(config.max_concurrent_requests);
    run_scrapers(&config, &rate_limiter, connector_func, &pool).await?;
    info!("All done");
    pool.close().await;
    Ok(())
}

async fn run_scrapers<T: Future<Output = Result<Html>> + Send>(
    cfg: &ScrapeConfig,
    rate_limiter: &RateLimiter,
    connector_func: fn(RequestClient, String) -> T,
    pool: &PgPool,
) -> Result<()> {
    let mut db_products;

    db_products = run_scraper(
        cfg, 
        AlbertHeijnScraper::new(connector_func), 
        rate_limiter, 
        "Albert Heijn")
        .await?;
    
    info!("Writing new scrapes to db...");
    tables::products::insert(&db_products, pool).await?;

    db_products = run_scraper(
        cfg, 
        JumboScraper::new(connector_func), 
        rate_limiter, 
        "Jumbo")
        .await?;

    info!("Writing new scrapes to db...");
    tables::products::insert(&db_products, pool).await?;

    Ok(())
}

async fn run_scraper(
    cfg: &ScrapeConfig, 
    scraper: impl Scraper,
    rate_limiter: &RateLimiter,
    scraper_name: &str,
) -> Result<Vec<InDbProduct>> {
    let products = scraper.scrape(cfg.max_items, rate_limiter).await?;
    Ok(
        products
        .into_iter()
        .map(|p| 
            InDbProduct::new(scraper_name.to_string(), p)
        )
        .collect()
    )
}