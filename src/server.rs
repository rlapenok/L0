use std::{env, error::Error};
use clap::Parser;
use axum::{
    extract::{MatchedPath, Request},
    serve,
};

use server::{
    app::create_app,
    config::AppConfig,
    infrastructure::services::{graceful_shutdown_server, ServerSt},
};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::info_span;



#[derive(Parser)]
struct Cli{
    address:String
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let address=Cli::parse().address;
    /*let args = env::args().collect::<Vec<String>>();
    args[1].trim().find("address").expect("Pass flag 'address'");
    let address = args[2].trim();*/

    let app_config = AppConfig::load()?;
    app_config.build_tracing()?;
    let state = app_config.to_state().await?;
    let shutdown = graceful_shutdown_server(state.clone());

    let app = create_app::<ServerSt>()
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
