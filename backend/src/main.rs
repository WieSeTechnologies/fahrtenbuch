mod routes;

use axum::response::Redirect;
use axum::routing::get;
use axum::Router;
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
    info!("THE DATABASE NEEDS TO BE RUNNING IN ORDER FOR THE BACKEND TO FUNCTION!");
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
        .layer(cors);

    info!("Server started successfully on 0.0.0.0:9000");

    // Set up the listener and start the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:9000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
