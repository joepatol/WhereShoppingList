use jumbo::scrape_jumbo;
use scrape_core::ConfigBuilder;
use sql;

const MAX_ITEMS_PER_SCRAPE: u32 = 48;

#[tokio::main]
async fn main() {
    println!("Starting scrape");
    let pool = sql::connect().await;
    let config = ConfigBuilder::new()
        .scrape_max_items(MAX_ITEMS_PER_SCRAPE)
        .build();

    sql::truncate_products(&pool).await;

    let products = scrape_jumbo(&config).await;
    for product in products.iter() {
        sql::insert_product(product, &pool).await;
    }

    println!("Scrape finished succesfully");
}