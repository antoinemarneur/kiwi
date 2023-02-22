use crate::{Result};
use crate::router::server::ApiContext;
use serde::{Deserialize, Serialize};
use axum::{
    Json,
    extract::{Path, Extension},
};
use crate::message::messages;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Like {
    id: String,
    message_id: String,
}

impl Like {
    fn new(id:String, message_id: String) -> Like {
        Like {
            id,
            message_id,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct LikeRequest {
    message_id: String,
}

pub async fn create_like(ctx: Extension<ApiContext>, Path(id): Path<String>) -> Result<Json<Like>> {
    let message = sqlx::query_as::<_, messages::Message>("SELECT * FROM message WHERE message.id=$1")
    .bind(&id)
    .fetch_one(&ctx.db)
    .await
    .unwrap();

    let like = Like::new(message.author_id ,id);

    let id = sqlx::query("INSERT INTO like VALUES ($1, $2)")
    .bind(&like.id)
    .bind(&like.message_id)
    .fetch_all(&ctx.db)
    .await
    .unwrap();

    Ok(Json(like))
}

pub async fn list_likes(ctx: Extension<ApiContext>, Path(id): Path<String>) -> Result<Json<Vec<Like>>> {
    let likes = sqlx::query_as::<_, Like>("SELECT * FROM like WHERE like.message_id=$1")
    .bind(id)
    .fetch_all(&ctx.db)
    .await
    .unwrap();

    Ok(Json(likes))
}