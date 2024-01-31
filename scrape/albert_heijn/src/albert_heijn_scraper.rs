use std::future::Future;
use std::num::ParseFloatError;
use anyhow::Result;
use scraper::Html;
use log::info;
use scrape_core::scrape_utils::{build_selector, walk_selectors};
use scrape_core::{ProductInfo, RateLimiter, ScrapeError, RequestClient, Scraper, RequestClientBuilder, request_header};

const BASE_URL: &str = "https://www.ah.nl";
const LETTER_URL: &str = "/producten/merk?letter=";
const LETTERS: [&str; 27] = [
    "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "%23",
];
const SRC: &str = "Albert Heijn";

pub fn src() -> String {
    SRC.to_string()
}

pub struct AlbertHeijnScraper<T> 
where
    T: Future<Output = Result<Html>> + Send,
{
    html_fetcher: fn(RequestClient, String) -> T
}

impl<T: Future<Output = Result<Html>> + Send> AlbertHeijnScraper<T> {
    pub fn new(fetch_func: fn(RequestClient, String) -> T) -> Self {
        Self { html_fetcher: fetch_func }
    }

    pub async fn scrape_brand_urls_for_letter(&self, letter: &str) -> Result<Vec<String>> {
        let mut headers = request_header::HeaderMap::new();
        headers.insert(request_header::CONNECTION, request_header::HeaderValue::from_static("keep-alive"));
        headers.insert(request_header::HOST, request_header::HeaderValue::from_static("www.ah.nl"));
        let client = RequestClientBuilder::new().default_headers(headers).gzip(true).build()?;

        let mut url = String::default();
        for sub_str in [BASE_URL, LETTER_URL, letter] {
            url.push_str(sub_str);
        }

        info!("Scraping brand urls at {}", &url);
        let document = (self.html_fetcher)(client, url).await?;
        let source = src();

        let brand_links_selector = build_selector("div.brand-hub_links__E6cvr", &src())?;
        let brand_links_container = walk_selectors(document.root_element(), &[brand_links_selector], &source)?;
        let a_selector = build_selector("a", &src())?;
        let brand_links = brand_links_container.select(&a_selector);
        let links: Vec<String> = brand_links
            .into_iter()
            .map(|l| l
                .value()
                .attr("href")
                .ok_or(ScrapeError::InvalidStructureAssumed { src: src() }))
            .collect::<Result<Vec<&str>, ScrapeError>>()?
            .into_iter()
            .map(|e| {
                let mut url = BASE_URL.to_string();
                url.push_str(e);
                url
            })
            .collect();
        Ok(links)
    }

    pub async fn scrape_brand_page(&self, url: &str) -> Result<Vec<ProductInfo>> {
        info!("Scraping brand url {}", url);
        let mut headers = request_header::HeaderMap::new();
        headers.insert("CONNECTION", request_header::HeaderValue::from_static("keep-alive"));
        headers.insert("HOST", request_header::HeaderValue::from_static("www.ah.nl"));
        let client = RequestClientBuilder::new().default_headers(headers).gzip(true).build()?;

        let document = (self.html_fetcher)(client, url.to_owned()).await?;

        let product_container_selector = build_selector(
            "article", 
            SRC
        )?;
        let product_containers = document.select(&product_container_selector);

        let mut products = Vec::new();
        for product_container in product_containers.into_iter() {
            let info_selector = build_selector("a", SRC)?;
            let product_name = walk_selectors(product_container, &[info_selector], SRC)?
                .attr("title")
                .ok_or(ScrapeError::InvalidStructureAssumed { src: src() })?;
            let price_selector = build_selector("div.price-amount_root__Sa88q", SRC)?;
            let price_str: String = product_container
                .select(&price_selector)
                .map(|a| a.text().collect::<String>()).collect();

            // let price: f32 = price_str.parse().map_err(|e: ParseFloatError| {
            //     ScrapeError::FailedToParseStringValue{ src: src(), err: format!("{}, at {} product {}", e.to_string(), url, &product_name)}
            // })?;
            let rprice: Result<f32, ParseFloatError> = price_str.parse();
            match rprice {
                Ok(price) => products.push(ProductInfo::new(product_name.to_owned(), price)),
                Err(_) => {},
            }
        };

        Ok(products) 
    }
}

impl<T: Future<Output = Result<Html>> + Send> Scraper for AlbertHeijnScraper<T> {
    async fn scrape(&self, max_items: Option<usize>, rate_limiter: &RateLimiter) -> Result<Vec<ProductInfo>> {
        info!(target: SRC, "Start scraping");
        let max_nr_products: usize;
    
        match max_items {
            Some(value) => max_nr_products = value,
            None => max_nr_products = usize::MAX,
        };
        info!("Scraping max {} products", &max_nr_products);

        let mut link_futures = Vec::new();
        for letter in LETTERS {
            link_futures.push(self.scrape_brand_urls_for_letter(&letter));
        }

        let links = rate_limiter.run(link_futures).await?;
        
        let mut futures = Vec::new();

        for link in links.iter() {
            futures.push(self.scrape_brand_page(link));
        };

        Ok(rate_limiter.run(futures).await?)
    }
}