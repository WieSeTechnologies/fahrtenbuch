mod data_models;
mod routes;
mod util;

use axum::http::StatusCode;
use axum::routing::get;
use axum::Router;
use axum::{response::Redirect, routing::post};
use dotenvy::dotenv;
use std::env;
use std::sync::Arc;
use tokio::sync::OnceCell;
use tower_http::cors::CorsLayer;
#[allow(unused_imports)]
use tracing::{debug, error, info, trace, warn};

static DB: OnceCell<Arc<sqlx::Pool<sqlx::Postgres>>> = OnceCell::const_new();

/// Initialize the database pool and store it in OnceCell
async fn init_db() -> Result<(), sqlx::Error> {
    let db_url =
        std::env::var("DATABASE_URL").expect("The \"DATABASE_URL\" variable must be defined.");

    info!("Connecting to Database at: {:?}", db_url);

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    // Run migrations
    sqlx::migrate!("./migrations").run(&pool).await?;

    // Store the pool in a OnceCell, making it globally accessible
    DB.set(Arc::new(pool)).expect("DB OneCell is set");

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing for logging
    tracing_subscriber::fmt::init();

    // Load `.env`-file if it exists
    if let None = dotenv().ok() {
        error!("Could not load `.env` file")
    }

    // Initialize DB connection pool
    init_db().await?;

    // === Server setup ===
    let cors = CorsLayer::permissive();

    // Redirect URL (fallback if env variable is missing)
    let redirect_url = match env::var("REDIRECT_URL") {
        Ok(url) => url,
        Err(_) => String::from("https://www.astrago.de"),
    };

    // Define application routes and apply middleware
    let app = Router::new()
        .route("/", get(Redirect::temporary(&redirect_url)))
        .route("/status", get(routes::status::status))
        .route(
            "/api/setup/create_initial_user",
            post(routes::setup::create_initial_user::create_initial_user),
        )
        .route("/api/user/login", post(routes::user::login::get_session))
        .route(
            "/api/stats/user_count",
            get(routes::stats::user_count::get_user_count),
        )
        .route(
            "/api/user/verify_session",
            post(routes::user::verify_session::post_verify_session),
        )
        .route(
            "/api/user/invalidate_all_sessions",
            post(routes::user::invalidate_all_sessions::post_invalidate_all_sessions),
        )
        .layer(cors)
        .fallback(not_found_handler);

    info!("Server started successfully on 0.0.0.0:9000");

    // Set up the listener and start the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:9000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn not_found_handler() -> (StatusCode, String) {
    (
        StatusCode::NOT_FOUND,
        String::from("404 - Not found.\nThe content you requested was not found on the server."),
    )
}
