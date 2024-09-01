use std::error::Error;

use confique::Config as ConfigBuilder;
use repositories_config::RepositoriesConfig;

use crate::{
    domain::{
        remote_repositories::ToOrderPresentationRepository,
        services::remote_order_presentation_remote_service::{
            RemoteOrderRepresentationService, ToOrderRepresentationRemoteRepositoryService,
        },
    },
    infrastructure::services::{OrderPresentationState, RemoteSrv},
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
    pub async fn to_state(self) -> Result<OrderPresentationState<RemoteSrv>, Box<dyn Error>> {
        let order_presentation_repository = self.repositories_config.to_repository().await?;
        let order_presentation_repository_service = order_presentation_repository.to_service()?;
        let cloned = order_presentation_repository_service.clone();
        //run in other task read_row_data
        tokio::spawn(async move {
            cloned.read_and_save_row_data().await;
        });
        let state = OrderPresentationState::new(order_presentation_repository_service);
        Ok(state)
    }
}
