use std::error::Error;

use axum::{routing::post, serve, Router};
use config::AppConfig;
use handlers::add_order::add_order;
use tokio::net::TcpListener;

mod config;
mod contracts;
mod converter;
mod data_saver;
mod handlers;
mod model;
mod repo;
mod state;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app_config = AppConfig::load().unwrap();
    //load from file
    let b = app_config.to_state().await?;

    let app = Router::new()
        .route("/add_order", post(add_order))
        .with_state(b);
    let lister = TcpListener::bind("localhost:8080").await?;
    serve(lister, app).await?;
    Ok(())
}
