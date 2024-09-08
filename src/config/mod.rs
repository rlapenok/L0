use std::error::Error;

use confique::Config as ConfigBuilder;
use repositories_config::RepositoriesConfig;
use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, registry, util::SubscriberInitExt, Layer};

use crate::{
    domain::services::{
        local_order_presentation_remote_services::ToLocalOrderRepresentationService,
        remote_order_presentation_remote_service::ToRemoteOrderRepresentationService,
        OrderPresentationState,
    },
    infrastructure::services::{LocalSrv, RemoteSrv, ServerState},
    utils::tracing_app::{build_env_filters, build_stdout_tracing_layer},
};

mod repositories_config;

#[derive(ConfigBuilder, Debug)]
pub struct AppConfig {
    #[config(nested)]
    repositories_config: RepositoriesConfig,
}

impl AppConfig {
    pub fn load() -> Result<Self, Box<dyn Error>> {
        //todo get path from env
        let app_config = Self::from_file("./config.toml")?;
        Ok(app_config)
    }
    pub fn build_tracing(&self) -> Result<(), Box<dyn Error>> {
        let filter = build_env_filters(Level::INFO)?;
        let std_out_tracing = build_stdout_tracing_layer().with_filter(filter);
        registry().with(std_out_tracing).init();
        Ok(())
    }
    pub async fn to_state(&self) -> Result<ServerState<RemoteSrv, LocalSrv>, Box<dyn Error>> {
        //convert config to local service
        let local_service = self.repositories_config.to_local_service().await?;
        //convert config to remote service
        let remote_service = self.repositories_config.to_remote_service().await?;
        //create state for server
        let state = ServerState::new(remote_service, local_service);
        state.save_raw_orders().await;
        Ok(state)
    }
}
