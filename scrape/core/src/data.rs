#[derive(Debug)]
pub struct InDbProduct {
    pub store: String,
    pub info: ProductInfo,
}

impl InDbProduct {
    pub fn new(store: String, info: ProductInfo) -> Self {
        Self { store, info }
    }

    pub fn db_search_string(&self) -> String {
        self.info.name.to_lowercase()
    }
}

#[derive(Debug)]
pub struct ProductInfo {
    pub name: String,
    pub price: f32,
    pub url: String,
}

impl ProductInfo {
    pub fn new(name: String, price: f32, url: String) -> Self {
        ProductInfo { name, price, url }
    }
}

#[derive(Debug)]
pub struct InDbError {
    pub scraper: String,
    pub message: String,
}

impl InDbError {
    pub fn new(scraper: String, message: String) -> Self {
        InDbError { scraper, message }
    }
}