use crate::{
    Result,
    Error,
    ResultExt,
    router::{
        server::ApiContext,
        extractor::AuthUser,
    },
};
use anyhow::Context;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use axum::{
    Json,
    extract::{Path, Extension},
};
use time::PrimitiveDateTime;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash};

#[derive(Debug, Serialize)]
pub struct User {
    username: String,
    email: String,
    token: String,
    bio: String,
    image: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UserProfile {
    username: String,
    bio: String,
    image: Option<String>,
    created_at: PrimitiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct UserRequest {
    username: String,
    email: String,
    password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginUser {
    email: String,
    password: String,
}

#[derive(serde::Deserialize, Default, PartialEq, Eq)]
#[serde(default)] // fill in any missing fields with `..UpdateUser::default()`
pub struct UpdateUser {
    username: Option<String>,
    email: Option<String>,
    password: Option<String>,
    bio: Option<String>,
    image: Option<String>,
}

pub async fn create_user(
    ctx: Extension<ApiContext>,
    Json(req): Json<UserRequest>
) -> Result<Json<User>> {
    let id = Uuid::new_v4();
    let password_hash = hash_password(req.password).await?;

    let user_id = sqlx::query_scalar!(
        r#"
            insert into user (id, username, email, password_hash)
            values ($1, $2, $3, $4)
            returning
                id as "id!: Uuid"
        "#,
        id,
        req.username,
        req.email,
        password_hash
    )
    .fetch_one(&ctx.db)
    .await
    .on_constraint("user_username_key", |_| {
        Error::unprocessable_entity([("username", "username taken")])
    })
    .on_constraint("user_email_key", |_| {
        Error::unprocessable_entity([("email", "email taken")])
    })?;

    Ok(Json(
        User {
            username: req.username,
            email: req.email,
            token: AuthUser { user_id }.to_jwt(&ctx),
            bio: "".to_string(),
            image: None,
        }
    ))
}

pub async fn login_user(
    ctx: Extension<ApiContext>,
    Json(req): Json<LoginUser>
) -> Result<Json<User>> {
    let user = sqlx::query!(
        r#"
            select 
                id as "id!: Uuid",
                username,
                email,
                bio,
                image,
                password_hash
            from user where email = $1
        "#,
        req.email
    )
    .fetch_optional(&ctx.db)
    .await?
    .ok_or(Error::unprocessable_entity([("email", "does not exist")]))?;

    verify_password(req.password, user.password_hash).await?;

    Ok(Json(
        User {
            username: user.username,
            email: user.email,
            token: AuthUser { user_id: user.id }.to_jwt(&ctx),
            bio: user.bio,
            image: user.image,
        }
    ))
}

pub async fn get_current_user(
    auth_user: AuthUser,
    ctx: Extension<ApiContext>
) -> Result<Json<User>> {
    let user = sqlx::query!(
        r#"
            select email, username, bio, image 
            from user where id = $1
        "#,
        auth_user.user_id
    )
    .fetch_one(&ctx.db)
    .await?;

    Ok(Json(
        User {
            username: user.username,
            email: user.email,
            token: auth_user.to_jwt(&ctx),
            bio: user.bio,
            image: user.image,
        },
    ))
}

pub async fn update_user(
    auth_user: AuthUser,
    ctx: Extension<ApiContext>,
    Json(req): Json<UpdateUser>
) -> Result<Json<User>> {
    if req == UpdateUser::default() {
        return get_current_user(auth_user, ctx).await;
    }

    let password_hash = if let Some(password) = req.password {
        Some(hash_password(password).await?)
    } else {
        None
    };

    let user = sqlx::query!(
        r#"
            update user
            set username = coalesce($1, user.username),
                email = coalesce($2, user.email),
                password_hash = coalesce($3, user.password_hash),
                bio = coalesce($4, user.bio),
                image = coalesce($5, user.image)
            where id = $6;
            select username, email, bio, image 
            from user
            where id = $6
        "#,
        req.username,
        req.email,
        password_hash,
        req.bio,
        req.image,
        auth_user.user_id,
        auth_user.user_id
    )
    .fetch_one(&ctx.db)
    .await
    .on_constraint("user_username_key", |_| {
        Error::unprocessable_entity([("username", "username taken")])
    })
    .on_constraint("user_email_key", |_| {
        Error::unprocessable_entity([("email", "email taken")])
    })?;
    
    Ok(Json(
        User {
            username: user.username,
            email: user.email,
            token: auth_user.to_jwt(&ctx),
            bio: user.bio,
            image: user.image,
        }
    ))
}

pub async fn get_user(
    _: AuthUser,
    ctx: Extension<ApiContext>,
    Path(id): Path<Uuid>
) -> Result<Json<UserProfile>> {
    let user = sqlx::query_as!(
        UserProfile,
        r#"
            select 
                username,
                bio,
                image,
                created_at as "created_at!: PrimitiveDateTime"
            from user
            where id = $1
        "#,
        id
    )
    .fetch_optional(&ctx.db)
    .await?
    .ok_or(Error::NotFound)?;

    Ok(Json(user))
}

async fn hash_password(password: String) -> Result<String> {
    // Argon2 hashing is designed to be computationally intensive,
    // so we need to do this on a blocking thread.
    Ok(tokio::task::spawn_blocking(move || -> Result<String> {
        let salt = SaltString::generate(rand::thread_rng());
        Ok(
            PasswordHash::generate(Argon2::default(), password, &salt)
                .map_err(|e| anyhow::anyhow!("failed to generate password hash: {}", e))?
                .to_string(),
        )
    })
    .await
    .context("panic in generating password hash")??)
}

async fn verify_password(password: String, password_hash: String) -> Result<()> {
    Ok(tokio::task::spawn_blocking(move || -> Result<()> {
        let hash = PasswordHash::new(&password_hash)
            .map_err(|e| anyhow::anyhow!("invalid password hash: {}", e))?;

        hash.verify_password(&[&Argon2::default()], password)
            .map_err(|e| match e {
                argon2::password_hash::Error::Password => Error::Unauthorized,
                _ => anyhow::anyhow!("failed to verify password hash: {}", e).into(),
            })
    })
    .await
    .context("panic in verifying password hash")??)
}