use crate::{
    Result,
    Error,
    ResultExt,
    router::{
        server::ApiContext,
        extractor::AuthUser,
    }
};
use uuid::Uuid;
use time::PrimitiveDateTime;
use serde::{Deserialize, Serialize};
use axum::{
    Json,
    extract::{Path, Extension},
};

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Message {
    id: Uuid,
    author_id: Uuid,
    created_at: PrimitiveDateTime,
    message: String,
    message_parent_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct MessageRequest {
    message: String,
}

pub async fn get_messages(
    ctx: Extension<ApiContext>
) -> Result<Json<Vec<Message>>> {
    let messages = sqlx::query_as!(
        Message,
        r#"
            select 
                id as "id!: Uuid",
                author_id as "author_id!: Uuid",
                created_at as "created_at!: PrimitiveDateTime",
                message,
                message_parent_id as "message_parent_id!: Option<Uuid>"
            from message
        "#
    )
    .fetch_all(&ctx.db)
    .await?;

    Ok(Json(messages))
}

pub async fn create_message(
    auth_user: AuthUser,
    ctx: Extension<ApiContext>,
    Json(input): Json<MessageRequest>
) -> Result<Json<Message>> {
    let message_id = Uuid::new_v4();

    let message = sqlx::query!(
        r#"
            insert into message (id, author_id, message)
            values ($1, $2, $3)
            returning
                id as "id!: Uuid",
                author_id as "author_id!: Uuid",
                created_at as "created_at!: PrimitiveDateTime",
                message as "message!",
                message_parent_id as "message_parent_id!: Option<Uuid>"
        "#,
        message_id,
        auth_user.user_id,
        input.message
    )
    .fetch_one(&ctx.db)
    .await
    .on_constraint("message", |_| {
        Error::unprocessable_entity([("message", format!("duplicate message id"))])
    })?;
    
    Ok(Json(
        Message { 
            id: message.id,
            author_id: message.author_id,
            created_at: message.created_at,
            message: message.message,
            message_parent_id: message.message_parent_id 
        }
    ))
}

pub async fn get_message(
    _: AuthUser,
    ctx: Extension<ApiContext>,
    Path(id): Path<Uuid>
) -> Result<Json<Message>> {
    let message = sqlx::query!(
        r#"
            select 
                id as "id!: Uuid",
                author_id as "author_id!: Uuid",
                created_at as "created_at!: PrimitiveDateTime",
                message as "message!",
                message_parent_id as "message_parent_id!: Option<Uuid>"
            from message
            where message.id = $1
        "#,
        id
    )
    .fetch_optional(&ctx.db)
    .await?
    .ok_or(Error::NotFound)?;

    Ok(Json(
        Message { 
            id: message.id,
            author_id: message.author_id,
            created_at: message.created_at,
            message: message.message,
            message_parent_id: message.message_parent_id 
        }
    ))
}

pub async fn delete_message(
    auth_user: AuthUser,
    ctx: Extension<ApiContext>,
    Path(id): Path<Uuid>
) -> Result<()> {
    sqlx::query!(
        r#"
            delete from message
            where message.id = $1 and author_id = $2
        "#,
        id,
        auth_user.user_id
    )
    .fetch_optional(&ctx.db)
    .await?;

    Ok(())
}

pub async fn create_comment(
    auth_user: AuthUser,
    ctx: Extension<ApiContext>,
    Path(id): Path<Uuid>,
    Json(input): Json<MessageRequest>
) -> Result<Json<Message>> {
    let message_id = Uuid::new_v4();

    let message = sqlx::query!(
        r#"
            insert into message (id, author_id, message, message_parent_id)
            values ($1, $2, $3, $4)
            returning
                id as "id!: Uuid",
                author_id as "author_id!: Uuid",
                created_at as "created_at!: PrimitiveDateTime",
                message as "message!",
                message_parent_id as "message_parent_id!: Option<Uuid>"
        "#,
        message_id,
        auth_user.user_id,
        input.message,
        id
    )
    .fetch_one(&ctx.db)
    .await
    .on_constraint("message", |_| {
        Error::unprocessable_entity([("message", format!("duplicate message id"))])
    })?;
    
    Ok(Json(
        Message { 
            id: message.id,
            author_id: message.author_id,
            created_at: message.created_at,
            message: message.message,
            message_parent_id: message.message_parent_id 
        }
    ))
}