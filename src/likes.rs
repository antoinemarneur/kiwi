use uuid::Uuid;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use axum::{
    Json,
    response::IntoResponse,
    http::{StatusCode, HeaderMap, Uri, header},
    extract::{State, Path},
};
use crate::messages;
use sqlx::sqlite::SqlitePool;

#[derive(Debug, Serialize, sqlx::FromRow)]
struct Likes {
    id: String,
    message_id: String,
}

impl Likes {
    fn new(id:String, message_id: String) -> Likes {
        Likes {
            id,
            message_id,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct LikeRequest {
    message_id: String,
}

pub async fn create_like(State(pool): State<SqlitePool>, Path(id): Path<String>) -> impl IntoResponse {
    let mut conn = pool.acquire().await.unwrap();

    let message = sqlx::query_as::<_, messages::Messages>("SELECT * FROM message WHERE message.id=$1")
    .bind(&id)
    .fetch_one(&mut conn)
    .await
    .unwrap();

    let like = Likes::new(message.author_id ,id);

    let id = sqlx::query("INSERT INTO like VALUES ($1, $2)")
    .bind(&like.id)
    .bind(&like.message_id)
    .fetch_all(&mut conn)
    .await
    .unwrap();

    Json(like)
}

pub async fn list_likes(State(pool): State<SqlitePool>, Path(id): Path<String>) -> impl IntoResponse {
    let mut conn = pool.acquire().await.unwrap();

    let likes = sqlx::query_as::<_, Likes>("SELECT * FROM like WHERE like.message_id=$1")
    .bind(id)
    .fetch_all(&mut conn)
    .await
    .unwrap();

    Json(likes)
}