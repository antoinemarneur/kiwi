use crate::user::users;
use axum::{
    routing::{get, post},
    Router,
};

pub fn router() -> Router {
    Router::new()
        .route(
            "/api/users",
            post(users::create_user)
        )
        .route(
            "/api/users/login",
            post(users::login_user)
        )
        .route(
            "/api/user",
            get(users::get_current_user)
            .put(users::update_user)
        )
        .route(
            "/api/user/:id",
            get(users::get_user)
        )
}