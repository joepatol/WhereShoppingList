use sqlx::PgPool;
use anyhow::Result;

pub async fn truncate_all(pool: &PgPool) -> Result<()> {
    products::truncate(pool).await?;
    scrape_errors::truncate(pool).await?;
    Ok(())
}

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
                url,
                searchstr
            ) 
            SELECT * FROM UNNEST(
                $1::VARCHAR(255)[], 
                $2::FLOAT4[], 
                $3::VARCHAR(100)[], 
                $4::VARCHAR(750)[],
                $5::VARCHAR(255)[]
            )";

        let names: Vec<&str> = products.iter().map(|p| p.info.name.as_str()).collect();
        let prices: Vec<f32> = products.iter().map(|p| p.info.price).collect();
        let stores: Vec<&str> = products.iter().map(|p| p.store.as_str()).collect();
        let urls: Vec<&str> = products.iter().map(|p: &InDbProduct| p.info.url.as_str()).collect();
        let search_strings: Vec<String> = products.iter().map(|p| p.db_search_string()).collect();
        
        sqlx::query_as::<_, ()>(query_str)
            .bind(names)
            .bind(prices)
            .bind(stores)
            .bind(urls)
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
        let query_str = "INSERT INTO products (name, price, store, url, searchstr) VALUES ($1, $2, $3, &4)";
    
        sqlx::query(query_str)
            .bind(&product.info.name)
            .bind(&product.info.price)
            .bind(&product.store)
            .bind(&product.info.url)
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

pub mod scrape_errors {
    use sqlx::PgPool;
    use scrape_core::{InDbError, DbError};
    use anyhow::Result;

    pub async fn insert(errors: &Vec<InDbError>, pool: &PgPool) -> Result<()>{
        let query_str = r"
            INSERT INTO scrape_errors(
                scraper, 
                message
            ) 
            SELECT * FROM UNNEST(
                $1::VARCHAR(255)[], 
                $2::TEXT[]
            )";

        let scrapers: Vec<&str> = errors.iter().map(|e| e.scraper.as_str()).collect();
        let messages: Vec<&str> = errors.iter().map(|e| e.message.as_str( )).collect();
        
        sqlx::query_as::<_, ()>(query_str)
            .bind(scrapers)
            .bind(messages)
            .fetch_all(pool)
            .await
            .map_err(|e| DbError::QueryFailed{ 
                query: query_str.to_string(),
                err: e.to_string()
            })?;
        Ok(())
    }

    pub async fn truncate(pool: &PgPool) -> Result<()> {
        let query_str = "TRUNCATE TABLE scrape_errors";

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