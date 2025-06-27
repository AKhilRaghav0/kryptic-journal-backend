use axum::{
    http::StatusCode,
    middleware,
    response::Json,
    routing::{get, post},
    Router,
};
use dotenvy::dotenv;
use serde_json::{json, Value};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::net::SocketAddr;
use tracing::{info, Level};
use tracing_subscriber;

mod auth;
mod db;
mod routes;
mod utils;

use auth::jwt::auth_middleware;
use routes::{auth as auth_routes, journal as journal_routes};

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    // Load environment variables
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in environment");

    // Create database connection pool
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await?;

    // Run migrations
    sqlx::migrate!("./migrations").run(&pool).await?;

    let app_state = AppState { db: pool };

    // Build our application with routes
    let protected_routes = Router::new()
        .route("/entries", post(journal_routes::create_entry))
        .route("/entries", get(journal_routes::get_entries))
        .route("/entries/:id", get(journal_routes::get_entry))
        .route("/entries/:id", axum::routing::put(journal_routes::update_entry))
        .route("/entries/:id", axum::routing::delete(journal_routes::delete_entry))
        .layer(middleware::from_fn_with_state(app_state.clone(), auth_middleware));

    let app = Router::new()
        .route("/health", get(health_check))
        // Auth routes (no middleware)
        .route("/register", post(auth_routes::register))
        .route("/login", post(auth_routes::login))
        // Merge protected routes
        .merge(protected_routes)
        .with_state(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("🚀 Kryptic Journal API listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check() -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({
        "status": "healthy",
        "service": "kryptic-journal-backend"
    })))
}
