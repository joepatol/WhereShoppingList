pub mod products {
    use sqlx::PgPool;
    use scrape_core::InDbProduct;
    
    pub async fn insert_one(product: &InDbProduct, pool: &PgPool) {
        let query = "INSERT INTO products (name, price, store, searchstr) VALUES ($1, $2, $3, &4)";
    
        sqlx::query(query)
            .bind(&product.info.name)
            .bind(&product.info.price)
            .bind(&product.store)
            .bind(&product.db_search_string())
            .execute(pool)
            .await
            .unwrap();
    }
    
    pub async fn truncate(pool: &PgPool) {
        let query = "TRUNCATE TABLE products";
        sqlx::query(query).execute(pool).await.unwrap();
    }
}