use dotenv::dotenv;
use std::env;

pub struct Config {
    pub database_url: String,
    pub max_db_connections: u32,
    pub app_port: u32,
}

pub fn init_logging() {
    tracing_subscriber::fmt().with_env_filter("info").init();
}

pub fn load_config() -> Config {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let max_db_connections: u32 = env::var("MAX_DB_CONNECTIONS")
        .unwrap_or_else(|_| "5".to_string())
        .parse()
        .expect("MAX_DB_CONNECTIONS must be a valid number");

    let app_port: u32 = env::var("APP_PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("APP_PORT must be a valid number");

    Config {
        database_url,
        max_db_connections,
        app_port,
    }
}
