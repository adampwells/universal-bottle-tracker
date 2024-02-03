mod error_wrapper;

use axum::{
    routing::{get, put},
    Json, Router,
};
use axum::extract::{Path, State};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::sqlite::SqliteQueryResult;
use sqlx::{Sqlite, SqlitePool};
use sqlx::migrate::MigrateDatabase;
use tower_http::services::{ServeDir, ServeFile};
use tracing::debug;
use crate::error_wrapper::AppError;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let db_url = String::from("sqlite://database/sqlite.db");
    if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
        Sqlite::create_database(&db_url).await.unwrap();
        match create_schema(&db_url).await {
            Ok(_) => debug!("Database created successfully"),
            Err(e) => panic!("{}",e),
        }
    }
    let pool = SqlitePool::connect(&db_url).await.unwrap();

    // build our application with a route
    let app = Router::new()
        .route("/api/bottle/:bottle_id", get(find_bottle_data))
        .route("/api/bottle", put(save_bottle_data))
        .route("/health", get(health_check))
        .nest_service("/", ServeDir::new("static"))
        .with_state(pool);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Bottle {
    bottle_id: String,
    batch_id: String,
    batch_name: String,
    batch_description: String,
    saved_by: String,
    saved_date: Option<NaiveDateTime>,
}

pub async fn find_bottle_data(State(pool): State<SqlitePool>, Path(bottle_id): Path<String>) -> Result<Json<Value>, AppError> {
    let json_payload = serde_json::json!({
        "message": format!("data for bottle {}", bottle_id),
    });
    Ok(Json(json_payload))
}

pub async fn save_bottle_data(State(pool): State<SqlitePool>, Json(bottle): Json<Bottle>) -> Result<Json<Value>, AppError> {
    let json_payload = serde_json::json!({
        "message": format!("saved a bottle with id {}", bottle.bottle_id),
    });
    Ok(Json(json_payload))
}

pub async fn health_check() -> Result<(), AppError> {
    Ok(())
}

async fn create_schema(db_url:&str) -> Result<SqliteQueryResult, sqlx::Error> {
    let pool = SqlitePool::connect(&db_url).await?;
    let qry =
        "PRAGMA foreign_keys = ON ;
    CREATE TABLE IF NOT EXISTS bottles
        (
            bottle_id                   TEXT PRIMARY KEY NOT NULL,
            batch_id                    TEXT                NOT NULL,
            batch_name                  TEXT                NOT NULL,
            batch_description           TEXT                NOT NULL,
            saved_by                    TEXT                NOT NULL,
            saved_date                  DATETIME DEFAULT (datetime('now','localtime'))
        );";
    let result = sqlx::query(&qry).execute(&pool).await;
    pool.close().await;
    return result;
}