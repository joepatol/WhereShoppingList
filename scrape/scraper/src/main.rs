use jumbo::JumboScraper;
use std::collections::HashMap;
use scrape_core::{ConfigBuilder, ScrapeConfig, Scraper, get_html_document, InDbProduct};
use sql;

const MAX_ITEMS_PER_SCRAPE: u32 = 48;

fn assemble_scrapers() -> HashMap<&'static str, impl Scraper> {
    HashMap::from([
        ("Jumbo", JumboScraper::new(get_html_document))
    ])
}

async fn scrape_all(cfg: &ScrapeConfig, scrapers: HashMap<&'static str, impl Scraper>) -> Vec<InDbProduct> {
    let mut db_products: Vec<InDbProduct> = Vec::new();
    for (scraper_name, scraper) in scrapers.iter() {
        println!("Scraping '{}'", scraper_name);
        let products = scraper.scrape(cfg).await.unwrap();
        let in_db_products = 
            products
            .into_iter()
            .map(|p| 
                InDbProduct::new(scraper_name.to_string(), p)
            );
        
        db_products.extend(in_db_products);
    }
    db_products
}

#[tokio::main]
async fn main() {
    println!("Starting scrape...");
    println!("Setting up SqlPool connection");
    let pool = sql::connect().await;

    let config = 
        ConfigBuilder::new()
        .scrape_max_items(MAX_ITEMS_PER_SCRAPE)
        .build();
    println!("Using configuration: {:?}", &config);

    println!("Clearing tables");
    sql::truncate_products(&pool).await;

    println!("Assembling scrapers...");
    let scrapers = assemble_scrapers();
    println!("Scraping...");
    let db_products = scrape_all(&config, scrapers).await;

    println!("Writing new scrapes to db...");
    for product in db_products.iter() {
        sql::insert_product(&product, &pool).await;
    }

    println!("All done");
}
