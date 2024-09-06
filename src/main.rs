use std::{env, error::Error};

use axum::{
    extract::MatchedPath,
    http::Request,
    routing::{get, post},
    serve, Router,
};
use config::AppConfig;
use handlers::{add_order::add_order, get_orders::get_orders};
use infrastructure::services::graceful_shutdown_server;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::info_span;

mod config;
mod domain;
mod errors;
mod handlers;
mod infrastructure;
mod models;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let args=env::args().collect::<Vec<String>>();
    args[1].trim().find("address").expect("Pass flag 'address'");
    let address=args[2].trim();
    
    let app_config = AppConfig::load()?;
    app_config.build_tracing()?;
    //load from file
    let state = app_config.to_state().await?;
    let shutdown = graceful_shutdown_server(state.clone());
    let app = Router::new()
        .route("/add_order", post(add_order))
        .route("/get_order", get(get_orders))
        .layer(
            TraceLayer::new_for_http().make_span_with(|req: &Request<_>| {
                let path = req
                    .extensions()
                    .get::<MatchedPath>()
                    .map(MatchedPath::as_str);
                info_span!("Server::handler",method=?req.method(),path)
            }),
        )
        .with_state(state);
    let lister = TcpListener::bind(address).await?;
    serve(lister, app).with_graceful_shutdown(shutdown).await?;

    Ok(())
}
