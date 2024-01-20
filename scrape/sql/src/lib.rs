use sqlx::{PgPool, Row};
use scrape_core::ProductInfo;

const CONN_URL: &str = "postgres://postgresuser:postgrespwd@localhost:5432/supermarkt";

pub async fn connect() -> PgPool {
    PgPool::connect(CONN_URL).await.unwrap()
}

pub async fn insert_product(product: &ProductInfo, pool: &PgPool) {
    let query = "INSERT INTO products (name, price) VALUES ($1, $2)";

    sqlx::query(query)
        .bind(&product.name)
        .bind(&product.price)
        .execute(pool)
        .await
        .unwrap();
}

pub async fn truncate_products(pool: &PgPool) {
    let query = "TRUNCATE TABLE products";
    sqlx::query(query).execute(pool).await.unwrap();
}

pub async fn get_a_product(pool: &PgPool) ->  ProductInfo {
    let query = "SELECT * FROM products";

    let data = sqlx::query(query).fetch_one(pool).await.unwrap();
    ProductInfo::new(
        data.get("name"),
        data.get("price"),
    )
}