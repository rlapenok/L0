use std::error::Error;

use confique::Config as ConfigBuilder;
use repositories_config::RepositoriesConfig;
use tracing_subscriber::{layer::SubscriberExt, registry, util::SubscriberInitExt};

use crate::{
    domain::services::{
        local_order_presentation_remote_services::ToLocalOrderRepresentationService,
        remote_order_presentation_remote_service::ToRemoteOrderRepresentationService,
    },
    infrastructure::services::{LocalSrv, OrderPresentationState, RemoteSrv}, utils::tracing_app::build_stdout_tracing_layer,
};

mod repositories_config;
mod server_config;

#[derive(ConfigBuilder, Debug)]
pub struct AppConfig {
    #[config(nested)]
    repositories_config: RepositoriesConfig,
}

impl AppConfig {
    pub fn load() -> Result<Self, Box<dyn Error>> {
        //todo get path from env
        let app_config = Self::from_file("config.toml")?;
        //println!("{:?}",app_config);
        Ok(app_config)
    }
    pub fn build_tracing(&self){

        let std_out_tracing=build_stdout_tracing_layer();
        registry().with(std_out_tracing).init();

    }
    pub async fn to_state(
        self,
    ) -> Result<OrderPresentationState<RemoteSrv, LocalSrv>, Box<dyn Error>> {
        let local_service = self.repositories_config.to_local_service().await?;
        let remote_service = self.repositories_config.to_remote_service().await?;
        let state = OrderPresentationState::new(remote_service, local_service);
        Ok(state)
    }
}
