use axum::{
    extract::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use tracing_subscriber;

#[derive(Debug, Deserialize, Serialize)]
struct User {
    username: String,
    email: String,
}

// GET /health
async fn health_check() -> &'static str {
    "OK"
}

// POST /users
async fn create_user(Json(payload): Json<User>) -> Json<User> {
    Json(payload)
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_env_filter("info").init();

    let router = Router::new()
        .route("/health", get(health_check))
        .route("/users", post(create_user));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!(
        "Server running at http://{}",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, router).await.unwrap();
}
