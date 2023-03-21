use anyhow::Context;
use axum::{Extension, Router};
use tower::ServiceBuilder;
use sqlx::sqlite::SqlitePool;
use crate::config::Config;
use crate::message;
use crate::like;
use crate::user;
use std::sync::Arc;
use tower_http::trace::TraceLayer;

// Core type through which handler functions can access API state.
// This can be accessed by adding the parameter 'Extension<ApiContext>' 
// to a handler function's parameters.
#[derive(Clone)]
pub struct ApiContext {
    pub config: Arc<Config>,
    pub db: SqlitePool,
}

pub async fn serve(config: Config, db: SqlitePool) -> anyhow::Result<()> {
    // Build the core of our router with different layer.
    let app = router().layer(
        ServiceBuilder::new()
            .layer(Extension(ApiContext {
                config: Arc::new(config),
                db,
            }))
            // Enables logging. Use `RUST_LOG=tower_http=debug`
            .layer(TraceLayer::new_for_http())
    );

    axum::Server::bind(&"0.0.0.0:3000".parse()?)
        .serve(app.into_make_service())
        .await
        .context("error running HTTP server")
}

fn router() -> Router {
    message::routes::router()
        .merge(like::routes::router())
        .merge(user::routes::router())
}