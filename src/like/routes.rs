use crate::like::likes;
use axum::{
    routing::{get},
    Router,
};

pub fn router() -> Router {
    Router::new()
        .route("/message/:id/like", get(likes::get_likes).post(likes::create_like))
}