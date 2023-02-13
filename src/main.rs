use axum::{
    routing::{get, post, delete},
    Router,
};
use kiwi::{
    messages,
    likes
};
use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error>{
    let pool = SqlitePool::connect("sqlite:db").await?;

    // build our application with a single route
    let app = Router::new()
                .route("/messages", get(messages::list_messages).post(messages::create_message))
                .route("/message/:id", get(messages::list_message).delete(messages::delete_message))
                .route("/message/:id/like", get(likes::list_likes).post(likes::create_like))
                .with_state(pool);

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}