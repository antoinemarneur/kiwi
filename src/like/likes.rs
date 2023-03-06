use crate::{Result};
use crate::router::server::ApiContext;
use uuid::Uuid;
use time::PrimitiveDateTime;
use serde::{Serialize};
use axum::{
    Json,
    extract::{Path, Extension},
};

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Like {
    id: Uuid,
    message_id: Uuid,
    created_at: PrimitiveDateTime,
}

pub async fn create_like(
    ctx: Extension<ApiContext>,
    Path(id): Path<Uuid>
) -> Result<Json<Like>> {
    let like = sqlx::query_as::<_, Like>(
        r#"
            insert into like 
            values ($1, $2)
            returning
                id,
                message,
                created_at
        "#
    )
    .bind(Uuid::new_v4())
    .bind(id)
    .fetch_one(&ctx.db)
    .await
    .unwrap();

    Ok(Json(like))
}

pub async fn get_likes(
    ctx: Extension<ApiContext>,
    Path(id): Path<String>
) -> Result<Json<Vec<Like>>> {
    let likes = sqlx::query_as::<_, Like>(
        r#"
            select * from like 
            where like.message_id = $1
        "#
    )
    .bind(id)
    .fetch_all(&ctx.db)
    .await
    .unwrap();

    Ok(Json(likes))
}