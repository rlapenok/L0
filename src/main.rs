use std::error::Error;

use axum::{
    routing::{get, post},
    serve, Router,
};
use config::AppConfig;
use handlers::{add_order::add_order, get_orders::get_orders};
use tokio::net::TcpListener;

mod config;
mod domain;
mod errors;
mod handlers;
mod infrastructure;
mod model;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app_config = AppConfig::load()?;
    app_config.build_tracing();
    //load from file
    let b = app_config.to_state().await?;

    let app = Router::new()
        .route("/add_order", post(add_order))
        .route("/get_order", get(get_orders))
        .with_state(b);
    let lister = TcpListener::bind("localhost:8080").await?;
    serve(lister, app).await?;

    Ok(())
}
