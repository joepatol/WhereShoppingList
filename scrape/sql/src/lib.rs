use sqlx::{PgPool, Row};
use scrape_core::{ProductInfo, InDbProduct};

const CONN_URL: &str = "postgres://postgresuser:postgrespwd@localhost:5432/supermarkt";

pub async fn connect() -> PgPool {
    PgPool::connect(CONN_URL).await.unwrap()
}

pub async fn insert_product(product: &InDbProduct, pool: &PgPool) {
    let query = "INSERT INTO products (name, price, store) VALUES ($1, $2, $3)";

    sqlx::query(query)
        .bind(&product.info.name)
        .bind(&product.info.price)
        .bind(&product.store)
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