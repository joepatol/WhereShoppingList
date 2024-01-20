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