pub mod products {
    use sqlx::PgPool;
    use scrape_core::{InDbProduct, DbError};
    use anyhow::Result;

    pub async fn insert(products: &Vec<InDbProduct>, pool: &PgPool) -> Result<()>{
        let query_str = r"
            INSERT INTO products(
                name, 
                price, 
                store, 
                searchstr
            ) 
            SELECT * FROM UNNEST(
                $1::VARCHAR(255)[], 
                $2::FLOAT4[], 
                $3::VARCHAR(100)[], 
                $4::VARCHAR(255)[]
            )";

        let mut names = Vec::new();
        let mut prices = Vec::new();
        let mut stores = Vec::new();
        let mut search_strings = Vec::new();

        for product in products.iter() {
            names.push(product.info.name.clone());
            prices.push(product.info.price.clone());
            stores.push(product.store.clone());
            search_strings.push(product.db_search_string());
        }
        
        sqlx::query_as::<_, ()>(query_str)
            .bind(names)
            .bind(prices)
            .bind(stores)
            .bind(search_strings)
            .fetch_all(pool)
            .await
            .map_err(|e| DbError::QueryFailed{ 
                query: query_str.to_string(),
                err: e.to_string()
            })?;
        Ok(())
    }

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
    
    pub async fn truncate(pool: &PgPool) -> Result<()> {
        let query = "TRUNCATE TABLE products";
        sqlx::query(query)
            .execute(pool)
            .await
            .map_err(|e| DbError::QueryFailed { 
                query: query.to_string(),
                err: e.to_string(), 
        })?;
        Ok(())
    }
}