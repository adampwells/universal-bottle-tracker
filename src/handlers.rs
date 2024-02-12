use axum::body::Body;
use axum::extract::{Path, State};
use axum::http::header;
use axum::Json;
use axum::response::{IntoResponse, Response};
use chrono::NaiveDateTime;
use nanoid::nanoid;
use serde_json::Value;
use sqlx::SqlitePool;
use crate::error_wrapper::AppError;
use crate::model::{Bottle, save_bottle, update_bottle};
use crate::labels::get_label_docx;

pub async fn get_or_create_bottle(State(pool): State<SqlitePool>, Path(bottle_id): Path<String>) -> Result<Json<Value>, AppError> {

    let bottles = sqlx::query_as!(
        Bottle,
        "SELECT * FROM bottles WHERE bottle_id = ?",
        bottle_id)
        .fetch_all(&pool)
        .await?;

    let bottle: Bottle;

    if bottles.is_empty() {
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

pub async fn update_bottle_data(State(pool): State<SqlitePool>, Json(bottle): Json<Bottle>) -> Result<Json<Value>, AppError> {
    let result = update_bottle(&pool, &bottle).await;
    if result.is_err() {
        return Err(AppError(result.err().unwrap().into()));
    }

    let json_payload = serde_json::json!({
        "message": format!("saved a bottle with id {}", bottle.bottle_id),
    });
    Ok(Json(json_payload))
}

pub async fn health_check() -> Result<(), AppError> {
    Ok(())
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

