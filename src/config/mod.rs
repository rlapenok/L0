use std::error::Error;

use confique::Config as ConfigBuilder;
use data_saver_config::DataSaverConfig;
use repositories_config::RepositoriesConfig;

use crate::{
    contracts::{repositories::ToRepositories, saver::ToSaver},
    state::state::{AppState, AppStateType},
};

mod data_saver_config;
mod repositories_config;
mod server_config;

#[derive(ConfigBuilder)]
pub struct AppConfig {
    #[config(nested)]
    repositories_config: RepositoriesConfig,
    #[config(nested)]
    data_saver_config: DataSaverConfig,
}

impl AppConfig {
    pub fn load() -> Result<Self, Box<dyn Error>> {
        //todo get path from env
        let app_config = Self::from_file("config.toml")?;
        Ok(app_config)
    }
    pub async fn to_state(&self) -> Result<AppStateType, Box<dyn Error>> 
    {
        let saver = self.data_saver_config.to_saver().await?;
        let repo = self.repositories_config.to_repositories().await?;
        let state = AppState::new(saver,repo);
        let _data = state.read_row_data().await?;
        Ok(state)
    }
}
