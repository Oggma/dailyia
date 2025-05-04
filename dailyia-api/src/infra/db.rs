use sqlx::postgres::PgPoolOptions;

pub async fn create_pool(
    database_url: &str,
    max_db_connections: u32,
) -> sqlx::Pool<sqlx::Postgres> {
    PgPoolOptions::new()
        .max_connections(max_db_connections)
        .connect(database_url)
        .await
        .expect("Failed to create pool.")
}
