use anyhow::Context;
use clap::Parser;
use sqlx::sqlite::SqlitePoolOptions;
use kiwi::config::Config;
use kiwi::router;

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    // Parse environment.
    // This will exit with a help message if something is wrong.
    let config = Config::parse();

    // Single connection pool for SQLx that will be shared across the whole application.
    // This helps to avoid opening a new connection for every API call. 
    let db = SqlitePoolOptions::new()
        .max_connections(50)
        .connect(&config.database_url)
        .await
        .context("could not connect to database_url")?;

    // Serve our application!
    router::server::serve(config, db).await?;

    Ok(())
}