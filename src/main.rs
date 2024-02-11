mod error_wrapper;
mod labels;

use axum::{
    routing::{get, put},
    Json, Router,
};
use axum::body::Body;
use axum::extract::{Path, State};
use axum::http::header;
use axum::response::{IntoResponse, Response};
use chrono::NaiveDateTime;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::sqlite::SqliteQueryResult;
use sqlx::{Sqlite, SqlitePool};
use sqlx::migrate::MigrateDatabase;
use tower_http::services::ServeDir;
use tracing::debug;
use crate::error_wrapper::AppError;
use crate::labels::get_label_docx;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let db_url = String::from("sqlite://sqlite.db");
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
        .route("/api/bottle/:bottle_id", get(get_or_create_bottle))
        .route("/api/bottle", put(save_bottle_data))
        .route("/health", get(health_check))
        .route("/labels", get(get_label_doc))
        .nest_service("/", ServeDir::new("static"))
        .with_state(pool);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[derive(Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Bottle {
    bottle_id: String,
    batch_id: String,
    batch_name: String,
    batch_description: String,
    saved_by: String,
    saved_date: Option<NaiveDateTime>,
}

pub async fn get_or_create_bottle(State(pool): State<SqlitePool>, Path(bottle_id): Path<String>) -> Result<Json<Value>, AppError> {

    let bottles = sqlx::query_as!(
        Bottle,
        "SELECT * FROM bottles WHERE bottle_id = ?",
        bottle_id)
            .fetch_all(&pool)
            .await?;

    let bottle: Bottle;

    if bottles.len() == 0 {
        bottle = Bottle {
            bottle_id: bottle_id.clone(),
            batch_id: String::from(""),
            batch_name: String::from(""),
            batch_description: String::from(""),
            saved_by: String::from(""),
            saved_date: Some(NaiveDateTime::default()),
        };
        save_bottle(&pool, &bottle).await?;
    } else {
        bottle = bottles[0].clone();
    }

    let json_payload = serde_json::json!({
        "message": format!("data for bottle {}", bottle_id),
        "data": bottle
    });
    Ok(Json(json_payload))
}

pub async fn get_label_doc() -> Result<Response, AppError> {

    let bytes = get_label_docx().await?;
    let body = Body::from(bytes);

    let headers = [
        (header::CONTENT_TYPE, "application/vnd.openxmlformats-officedocument.wordprocessingml.document"),
        (header::CONTENT_DISPOSITION,  &format!("attachment; filename=\"bottle_labels_{}\".docx", nanoid!())),
    ];

    Ok((headers, body).into_response())
}

async fn save_bottle(pool: &SqlitePool, bottle: &Bottle) -> Result<SqliteQueryResult, sqlx::Error> {
    let result = sqlx::query(
        "INSERT INTO bottles (bottle_id, batch_id, batch_name, batch_description, saved_by) VALUES (?, ?, ?, ?, ?)",
    )
    .bind(&bottle.bottle_id)
    .bind(&bottle.batch_id)
    .bind(&bottle.batch_name)
    .bind(&bottle.batch_description)
    .bind(&bottle.saved_by)
    .execute(pool)
    .await;
    return result;
}

pub async fn save_bottle_data(State(pool): State<SqlitePool>, Json(bottle): Json<Bottle>) -> Result<Json<Value>, AppError> {
    save_bottle(&pool, &bottle).await?;
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