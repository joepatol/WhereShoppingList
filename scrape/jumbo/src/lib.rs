use scraper::{Html, ElementRef};
use futures::future::join_all;
use scrape_core::{get_html_document, ProductInfo, ScrapeConfig};

const PRODUCTS_PER_PAGE: u32 = 24;
const URL: &str = "https://www.jumbo.com/producten";
const OFFSET_URL: &str = "/?offSet=";

pub async fn scrape_jumbo(cfg: &ScrapeConfig) -> Vec<ProductInfo> {
    let document = get_html_document(URL).await;
    let nr_pages = get_nr_pages(&document);
    let max_nr_products: u32;

    match cfg.max_items {
        Some(value) => max_nr_products = value,
        None => max_nr_products = nr_pages * PRODUCTS_PER_PAGE,
    };

    let mut loaded_nr_products = 0;
    let mut futures = Vec::new();

    while loaded_nr_products < max_nr_products {
        futures.push(scrape_page(loaded_nr_products.to_string()));
        loaded_nr_products += PRODUCTS_PER_PAGE;
    };

    join_all(futures).await.into_iter().flatten().collect()
}

fn get_nr_pages(document: &Html) -> u32 {
    let html_button_select = scraper::Selector::parse("button").unwrap();
    let html_page_bar_selector = scraper::Selector::parse("div.pagination").unwrap();
    let html_page_grid_selector = scraper::Selector::parse("div.pages-grid").unwrap();
    let mut html_page_bar = document.select(&html_page_bar_selector);
    let mut pages_grid = html_page_bar.next().unwrap().select(&html_page_grid_selector);
    
    let last: String = pages_grid
        .next()
        .unwrap()
        .select(&html_button_select)
        .last()
        .unwrap()
        .text()
        .collect();

    last.parse().unwrap()
}

async fn scrape_page(offset: String) -> Vec<ProductInfo> {
    let mut url = String::new();
    for slice in [URL, OFFSET_URL, &offset] {
        url.push_str(slice);
    }
    let document = get_html_document(&url).await;

    let html_product_selector = scraper::Selector::parse("article.product-container").unwrap();
    let html_products = document.select(&html_product_selector);

    html_products.map(|html_product| {
        ProductInfo::new(get_name(&html_product), get_price(&html_product))
    }).collect()
}

fn get_name(html_product: &ElementRef) -> String {
    html_product
        .select(&scraper::Selector::parse("div.content").unwrap())
        .next().unwrap()
        .select(&scraper::Selector::parse("div.upper").unwrap())
        .next().unwrap()
        .select(&scraper::Selector::parse("div.name").unwrap())
        .next().unwrap()
        .select(&scraper::Selector::parse("h2").unwrap())
        .next().unwrap()
        .select(&scraper::Selector::parse("a").unwrap())
        .map(|a| a.text().collect::<String>()).collect()
}

fn get_price(html_product: &ElementRef) -> f32 {
    let html_price = html_product
    .select(&scraper::Selector::parse("div.jum-price").unwrap())
    .next().unwrap()
    .select(&scraper::Selector::parse("div.current-price").unwrap())
    .next().unwrap();

    let mut whole_price: String = html_price
        .select(&scraper::Selector::parse("span.whole").unwrap())
        .map(|a| a.text().collect::<String>()).collect();
    let frac_price: String = html_price
        .select(&scraper::Selector::parse("sup.fractional").unwrap())
        .map(|a| a.text().collect::<String>()).collect();

    whole_price.push_str(".");
    whole_price.push_str(frac_price.as_str());

    whole_price.parse().unwrap()
}