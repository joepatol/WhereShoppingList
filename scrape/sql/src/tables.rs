pub mod products {
    use sqlx::PgPool;
    use scrape_core::{InDbProduct, DbError};
    use anyhow::{Ok, Result};

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

        let names: Vec<&str> = products.iter().map(|p| p.info.name.as_str()).collect();
        let prices: Vec<f32> = products.iter().map(|p| p.info.price).collect();
        let stores: Vec<&str> = products.iter().map(|p| p.store.as_str()).collect();
        let search_strings: Vec<String> = products.iter().map(|p| p.db_search_string()).collect();
        
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

    pub async fn insert_one(product: &InDbProduct, pool: &PgPool) -> Result<()>{
        let query_str = "INSERT INTO products (name, price, store, searchstr) VALUES ($1, $2, $3, &4)";
    
        sqlx::query(query_str)
            .bind(&product.info.name)
            .bind(&product.info.price)
            .bind(&product.store)
            .bind(&product.db_search_string())
            .execute(pool)
            .await
            .map_err(|e| DbError::QueryFailed{ 
                query: query_str.to_string(),
                err: e.to_string()
            })?;
        Ok(())
    }   
    
    pub async fn truncate(pool: &PgPool) -> Result<()> {
        let query_str = "TRUNCATE TABLE products";

        sqlx::query(query_str)
            .execute(pool)
            .await
            .map_err(|e| DbError::QueryFailed { 
                query: query_str.to_string(),
                err: e.to_string(), 
        })?;
        Ok(())
    }
}