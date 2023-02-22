use crate::{Result};
use crate::router::server::ApiContext;
use uuid::Uuid;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use axum::{
    Json,
    extract::{Path, Extension},
};

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Message {
    id: String,
    pub author_id: String,
    created_at: String, //DateTime<Utc>,
    message: String,
}

impl Message {
    fn new(message: String) -> Message {
        Message {
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

pub async fn list_messages(ctx: Extension<ApiContext>) -> Result<Json<Vec<Message>>> {
    let messages = sqlx::query_as::<_, Message>("SELECT * FROM message")
    .fetch_all(&ctx.db)
    .await
    .unwrap();

    Ok(Json(messages))
}

pub async fn create_message(ctx: Extension<ApiContext>, Json(input): Json<MessageRequest>) -> Result<Json<Message>> {
    let message = Message::new(input.message);

    let id = sqlx::query("INSERT INTO message VALUES ($1, $2, $3, $4)")
    .bind(&message.id)
    .bind(&message.author_id)
    .bind(&message.created_at)
    .bind(&message.message)
    .fetch_all(&ctx.db)
    .await
    .unwrap();
    
    Ok(Json(message))
}

pub async fn list_message(ctx: Extension<ApiContext>, Path(id): Path<String>) -> Result<Json<Message>> {
    let message = sqlx::query_as::<_, Message>("SELECT id, author_id, created_at, message FROM message WHERE message.id=$1")
    .bind(id)
    .fetch_one(&ctx.db)
    .await
    .unwrap();

    Ok(Json(message))
}

pub async fn delete_message(ctx: Extension<ApiContext>, Path(id): Path<String>) -> Result<()> {
    let message = sqlx::query("DELETE FROM message WHERE message.id=$1")
    .bind(id)
    .fetch_all(&ctx.db)
    .await
    .unwrap();

    Ok(())
}