use crate::message::messages;
use axum::{
    routing::{get},
    Router,
};

pub fn router() -> Router {
    Router::new()
                .route(
                    "/messages",
                    get(messages::get_messages)
                    .post(messages::create_message),
                )
                .route(
                    "/message/:id",
                    get(messages::get_message)
                    .delete(messages::delete_message)
                    .post(messages::create_comment),
                )
}