use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteQueryResult;
use sqlx::SqlitePool;

#[derive(Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Bottle {
    pub bottle_id: String,
    pub batch_id: String,
    pub batch_name: String,
    pub batch_description: String,
    pub saved_by: String,
    pub saved_date: Option<NaiveDateTime>,
}

pub async fn save_bottle(pool: &SqlitePool, bottle: &Bottle) -> Result<SqliteQueryResult, sqlx::Error> {
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

pub async fn update_bottle(pool: &SqlitePool, bottle: &Bottle) -> Result<SqliteQueryResult, sqlx::Error> {
    let result = sqlx::query(
        "UPDATE bottles SET batch_id = ?, batch_name = ?, batch_description = ?, saved_by = ? WHERE bottle_id = ?",
    )
        .bind(&bottle.batch_id)
        .bind(&bottle.batch_name)
        .bind(&bottle.batch_description)
        .bind(&bottle.saved_by)
        .bind(&bottle.bottle_id)
        .execute(pool)
        .await;
    return result;
}

pub async fn create_schema(db_url: &str) -> Result<SqliteQueryResult, sqlx::Error> {
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
    result
}