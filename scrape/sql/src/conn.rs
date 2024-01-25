use scrape_core::DbError;
use std::env;

pub async fn connect() -> anyhow::Result<sqlx::PgPool> {
    Ok(
        sqlx::PgPool::connect(&get_conn_string())
        .await
        .map_err(|e| DbError::FailedToConnect { err: e.to_string() })?
    )
}

fn get_conn_string() -> String {
    env::var("CONN_URL")
    .unwrap_or("postgres://postgresuser:postgrespwd@localhost:5432/supermarkt".to_owned())
}