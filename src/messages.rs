use uuid::Uuid;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use axum::{
    Json,
    response::IntoResponse,
    http::{StatusCode, HeaderMap, Uri, header},
    extract::{State, Path},
};
use sqlx::sqlite::SqlitePool;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Messages {
    id: String,
    pub author_id: String,
    created_at: String, //DateTime<Utc>,
    message: String,
}

impl Messages {
    fn new(message: String) -> Messages {
        Messages {
            id: Uuid::new_v4().to_string(),
            author_id: "1111".to_string(),
            created_at: Utc::now().to_string(),
            message,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct MessageRequest {
    message: String,
}

pub async fn list_messages(State(pool): State<SqlitePool>) -> impl IntoResponse {
    let mut conn = pool.acquire().await.unwrap();

    let messages = sqlx::query_as::<_, Messages>("SELECT * FROM message")
    .fetch_all(&mut conn)
    .await
    .unwrap();

    Json(messages)
}

pub async fn create_message(State(pool): State<SqlitePool>, Json(input): Json<MessageRequest>) -> impl IntoResponse {
    let mut conn = pool.acquire().await.unwrap();
    let message = Messages::new(input.message);

    let id = sqlx::query("INSERT INTO message VALUES ($1, $2, $3, $4)")
    .bind(&message.id)
    .bind(&message.author_id)
    .bind(&message.created_at)
    .bind(&message.message)
    .fetch_all(&mut conn)
    .await
    .unwrap();
    
    Json(message)
}

pub async fn list_message(State(pool): State<SqlitePool>, Path(id): Path<String>) -> impl IntoResponse {
    let mut conn = pool.acquire().await.unwrap();

    let message = sqlx::query_as::<_, Messages>("SELECT id, author_id, created_at, message FROM message WHERE message.id=$1")
    .bind(id)
    .fetch_one(&mut conn)
    .await
    .unwrap();

    Json(message)
}

pub async fn delete_message(State(pool): State<SqlitePool>, Path(id): Path<String>) -> impl IntoResponse {
    let mut conn = pool.acquire().await.unwrap();

    let message = sqlx::query("DELETE FROM message WHERE message.id=$1")
    .bind(id)
    .fetch_all(&mut conn)
    .await
    .unwrap();

    (StatusCode::OK)
}