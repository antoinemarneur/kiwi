use crate::{
    Result,
    router::{
        server::ApiContext,
        extractor::AuthUser,
    }
};
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
    auth_user: AuthUser,
    ctx: Extension<ApiContext>,
    Path(id): Path<Uuid>
) -> Result<Json<Like>> {
    let like = sqlx::query!(
        r#"
            insert into like (id, message_id)
            values ($1, $2)
            returning
                id as "id!: Uuid",
                message_id as "message_id!: Uuid",
                created_at as "created_at!: PrimitiveDateTime"
        "#,
        auth_user.user_id,
        id
    )
    .fetch_one(&ctx.db)
    .await
    .unwrap();

    Ok(Json(
        Like {
            id: like.id,
            message_id: like.message_id,
            created_at: like.created_at
        }
    ))
}

pub async fn get_likes(
    _: AuthUser,
    ctx: Extension<ApiContext>,
    Path(id): Path<String>
) -> Result<Json<Vec<Like>>> {
    let likes = sqlx::query_as!(
        Like,
        r#"
            select
                id as "id!: Uuid",
                message_id as "message_id!: Uuid",
                created_at as "created_at!: PrimitiveDateTime"
            from like 
            where like.message_id = $1
        "#,
        id
    )
    .fetch_all(&ctx.db)
    .await
    .unwrap();

    Ok(Json(likes))
}