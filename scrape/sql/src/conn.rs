use scrape_core::DbError;

const CONN_URL: &str = "postgres://postgresuser:postgrespwd@localhost:5432/supermarkt";

pub async fn connect() -> anyhow::Result<sqlx::PgPool> {
    Ok(
        sqlx::PgPool::connect(CONN_URL)
        .await
        .map_err(|e| DbError::FailedToConnect { err: e.to_string() })?
    )
}