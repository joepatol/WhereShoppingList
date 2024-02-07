use log::info;
use jumbo::JumboScraper;
use albert_heijn::AlbertHeijnScraper;
use scrape_core::{RandomDelayRateLimiter, RateLimiter, Scraper, SimpleRateLimiter};
use sql::{tables, self, PgPool};
use anyhow::Result;
use scrape_core::{InDbProduct, InDbError, ScrapeConfig, ReqwestHtmlLoader, RequestClient, request_header, RequestClientBuilder};

pub async fn scrape(config: ScrapeConfig) -> Result<()> {
    info!("Starting scrape...");
    info!("Setting up SqlPool connection");
    let pool = sql::connect().await?;
    info!("Using configuration: {:?}", &config);
    info!("Clearing tables");
    tables::truncate_all(&pool).await?;
    info!("Scraping...");
    run_scrapers(&config, &pool).await?;
    info!("All done");
    pool.close().await;
    Ok(())
}

async fn run_scrapers(
    cfg: &ScrapeConfig,
    pool: &PgPool,
) -> Result<()> {
    let mut db_products;
    let mut errors;

    let rate_limiter = SimpleRateLimiter::new(cfg.max_concurrent_requests);

    let jumbo_client = RequestClient::new();
    let jumbo_connector = ReqwestHtmlLoader::new(&jumbo_client);

    (db_products, errors) = run_scraper(
        cfg.max_requests, 
        JumboScraper::new(&jumbo_connector), 
        &rate_limiter, 
        "Jumbo")
        .await;

    info!("Done, got {} errors and {} successes", errors.len(), db_products.len());
    info!("Writing new scrapes to db...");
    tables::products::insert(&db_products, pool).await?;
    tables::scrape_errors::insert(&errors, pool).await?;

    let delay_rate_limiter = RandomDelayRateLimiter::new(
        cfg.max_concurrent_requests, 100, 5000,
    );

    let mut headers = request_header::HeaderMap::new();
    headers.insert(request_header::CONNECTION, request_header::HeaderValue::from_static("keep-alive"));
    headers.insert(request_header::HOST, request_header::HeaderValue::from_static("www.ah.nl"));
    let ah_client = RequestClientBuilder::new().default_headers(headers).gzip(true).build()?;

    let ah_connector = ReqwestHtmlLoader::new(&ah_client);

    (db_products, errors) = run_scraper(
        cfg.max_requests, 
        AlbertHeijnScraper::new(&ah_connector), 
        &delay_rate_limiter, 
        "Albert Heijn")
        .await;
    
    info!("Done, got {} errors and {} successes", errors.len(), db_products.len());
    info!("Writing new scrapes to db...");
    tables::products::insert(&db_products, pool).await?;
    tables::scrape_errors::insert(&errors, pool).await?;

    Ok(())
}

async fn run_scraper<R: RateLimiter + Send + Sync>(
    max_requests: Option<usize>, 
    scraper: impl Scraper,
    rate_limiter: &R,
    scraper_name: &str,
) -> (Vec<InDbProduct>, Vec<InDbError>) {
    let results = scraper.scrape(max_requests, rate_limiter).await;
    results.map_extract(
       |p| InDbProduct::new(scraper_name.to_string(), p),
       |e| InDbError::new(scraper_name.to_string(), e.to_string())
    )
}