use crate::{Result, Error, ResultExt};
use crate::router::server::ApiContext;
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
    pub author_id: Uuid,
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
    let messages = sqlx::query_as::<_, Message>(
        r#"
            select * from message
        "#
    )
    .fetch_all(&ctx.db)
    .await?;

    Ok(Json(messages))
}

pub async fn create_message(
    ctx: Extension<ApiContext>,
    Json(input): Json<MessageRequest>
) -> Result<Json<Message>> {
    let message = sqlx::query_as::<_, Message>(
        r#"
            insert into message (id, author_id, message)
            values ($1, $2, $3)
            returning
                id,
                author_id,
                created_at,
                message,
                message_parent_id
        "#
    )
    .bind(Uuid::new_v4())
    .bind(Uuid::new_v4())
    .bind(input.message)
    .fetch_one(&ctx.db)
    .await
    .on_constraint("message", |_| {
        Error::unprocessable_entity([("message", format!("duplicate message id"))])
    })?;
    
    Ok(Json(message))
}

pub async fn get_message(
    ctx: Extension<ApiContext>,
    Path(id): Path<Uuid>
) -> Result<Json<Message>> {
    let message = sqlx::query_as::<_, Message>(
        r#"
            select 
                id,
                author_id,
                created_at,
                message,
                message_parent_id
            from message
            where message.id = $1
        "#
    )
    .bind(id)
    .fetch_optional(&ctx.db)
    .await?
    .ok_or(Error::NotFound)?;

    Ok(Json(message))
}

pub async fn delete_message(
    ctx: Extension<ApiContext>,
    Path(id): Path<Uuid>
) -> Result<()> {
    sqlx::query(
        r#"
            delete from message
            where message.id = $1
        "#
    )
    .bind(id)
    .fetch_optional(&ctx.db)
    .await?;

    Ok(())
}

pub async fn create_comment(
    ctx: Extension<ApiContext>,
    Path(id): Path<Uuid>,
    Json(input): Json<MessageRequest>
) -> Result<Json<Message>> {
    let message = sqlx::query_as::<_, Message>(
        r#"
            insert into message (id, author_id, message, message_parent_id)
            values ($1, $2, $3, $4)
            returning
                id, 
                author_id,
                created_at,
                message,
                message_parent_id
        "#
    )
    .bind(Uuid::new_v4())
    .bind(Uuid::new_v4())
    .bind(input.message)
    .bind(id)
    .fetch_one(&ctx.db)
    .await
    .on_constraint("message", |_| {
        Error::unprocessable_entity([("message", format!("duplicate message id"))])
    })?;
    
    Ok(Json(message))
}