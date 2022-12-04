use axum::{
    routing::{get, post},
    Router,
};
use kiwi::messages;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
                .route("/messages", get(messages::list_messages).post(messages::create_message));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}