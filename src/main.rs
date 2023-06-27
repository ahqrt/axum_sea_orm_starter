use axum::{
    routing::{get, post},
    Router,
};
use sea_orm::{ConnectOptions, Database};
use std::time::Duration;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut options =
        ConnectOptions::new("postgres://postgres:postgres@localhost:5432/app_db".to_owned());

    options
        .max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(10))
        .acquire_timeout(Duration::from_secs(10))
        .idle_timeout(Duration::from_secs(10))
        .max_lifetime(Duration::from_secs(10))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Debug)
        .set_schema_search_path("schema".into());

    let db = Database::connect(options).await?;

    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
