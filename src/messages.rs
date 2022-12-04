use uuid::Uuid;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use axum::{
    Json,
    response::IntoResponse,
    http::{StatusCode, HeaderMap, Uri, header},
};

#[derive(Debug, Serialize)]
struct Messages {
    id: String,
    created_at: DateTime<Utc>,
    message: String,
}

impl Messages {
    fn new(message: String) -> Messages {
        Messages {
            id: Uuid::new_v4().to_string(),
            created_at: Utc::now(),
            message,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct MessageRequest {
    message: String,
}

pub async fn list_messages() -> impl IntoResponse {

}

pub async fn create_message(Json(input): Json<MessageRequest>) -> impl IntoResponse {
    let message = Messages::new(input.message);
    
    Json(message)
}