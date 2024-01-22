use std::collections::HashMap;
use scraper::Html;
use std::future::Future;
use jumbo::JumboScraper;
use scrape_core::Scraper;
use sql::{tables, connect};
use anyhow::Result;
use scrape_core::{InDbProduct, ScrapeConfig};

pub async fn scrape<T: Future<Output = Result<Html>>>(config: &ScrapeConfig, connector_func: fn(String) -> T) {
    println!("Starting scrape...");
    println!("Setting up SqlPool connection");
    let pool = connect().await.unwrap();
    println!("Using configuration: {:?}", &config);
    println!("Clearing tables");
    tables::products::truncate(&pool).await;
    println!("Assembling scrapers...");
    let scrapers = build_scrapers(connector_func);
    println!("Scraping...");
    let db_products_result = run_scrapers(&config, scrapers).await;
    match db_products_result {
        Ok(db_products) => {
            println!("Writing new scrapes to db...");
            for product in db_products.iter() {
                tables::products::insert_one(&product, &pool).await;
            }
            println!("All done");
        },
        Err(e) => {
            println!("Scraping failed, error: {}", e);
        }
    }
}

fn build_scrapers<T: Future<Output = Result<Html>>>(connector_func: fn(String) -> T) -> HashMap<&'static str, impl Scraper> {
    HashMap::from([
        ("Jumbo", JumboScraper::new(connector_func))
    ])
}

async fn run_scrapers(cfg: &ScrapeConfig, scrapers: HashMap<&'static str, impl Scraper>) -> Result<Vec<InDbProduct>> {
    let mut db_products: Vec<InDbProduct> = Vec::new();
    for (scraper_name, scraper) in scrapers.iter() {
        println!("Scraping '{}'", scraper_name);
        let products = scraper.scrape(cfg).await?;
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