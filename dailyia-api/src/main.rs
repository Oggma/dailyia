use axum::Router;

mod infra;
use crate::infra::config;
use crate::infra::db;

mod routes;
use crate::routes::health;

#[tokio::main]
async fn main() {
    config::init_logging();

    let config = config::load_config();

    let _pool = db::create_pool(&config.database_url, config.max_db_connections).await;

    let router = Router::new().route("/health", axum::routing::get(health::health_check));

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", config.app_port))
        .await
        .unwrap();
    println!(
        "Server running at http://{}",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, router).await.unwrap();
}
