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
}

impl ProductInfo {
    pub fn new(name: String, price: f32) -> Self {
        ProductInfo { name, price }
    }
}