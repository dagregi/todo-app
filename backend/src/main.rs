mod handlers;

use std::time::Duration;

use axum::{
    routing::{delete, get},
    Router,
};
use axum_error::Result;
use handlers::{create, delete_todo, read, update_todo};
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() -> Result<()> {
    let _ = dotenv::dotenv;
    let connection_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres:postgres.sql".to_string());
    let pool = PgPoolOptions::new()
        .max_connections(4)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&connection_url)
        .await
        .expect("Couldn't connect to database!");

    let app = Router::new()
        .route("/todos", get(read).post(create))
        .route("/todos/:id", delete(delete_todo).patch(update_todo))
        .with_state(pool)
        .layer(CorsLayer::very_permissive());

    Ok(axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await?)
}
