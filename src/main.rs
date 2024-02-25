use axum::{
    Router
    , routing::{get, put},
};

use sqlx::{Sqlite, SqlitePool};
use sqlx::migrate::MigrateDatabase;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
use tracing::debug;

use crate::handlers::{get_label_doc, get_or_create_bottle, health_check, update_bottle_data};
use crate::model::create_schema;

mod error_wrapper;
mod labels;
mod model;
mod handlers;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let db_url = String::from("sqlite://database/sqlite.db");
    if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
        Sqlite::create_database(&db_url).await.unwrap();
        match create_schema(&db_url).await {
            Ok(_) => debug!("Database created successfully"),
            Err(e) => panic!("{}", e),
        }
    }
    let pool = SqlitePool::connect(&db_url).await.unwrap();

    // build our application with a route
    let app = Router::new()
        .route("/api/bottle/:bottle_id", get(get_or_create_bottle))
        .route("/api/bottle", put(update_bottle_data))
        .route("/health", get(health_check))
        .route("/labels", get(get_label_doc))
        .nest_service("/", ServeDir::new("static"))
        .layer(CorsLayer::permissive())
        .with_state(pool);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

