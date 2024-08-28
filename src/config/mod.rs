use std::{error::Error, future::Future, sync::Arc, time::Duration};

use confique::Config as ConfigBuilder;
use data_saver_config::DataSaverConfig;
use postgres_config::PostgresConfig;
use tokio_postgres::{
    config::{Config, SslMode::Disable},
    NoTls,
};

use crate::{contracts::{repositories::ToRepositories, saver::{Saver, ToSaver}}, repo::postgres::PostgresRepo, state::state::AppState};

mod data_saver_config;
mod postgres_config;
mod server_config;

#[derive(ConfigBuilder)]
pub struct AppConfig {
    #[config(nested)]
    postgres_config: PostgresConfig,
    #[config(nested)]
    data_saver_config:DataSaverConfig
}

impl AppConfig {
    pub fn load() -> Result<Self, Box<dyn Error>> {
        //todo get path from env
        let app_config = Self::from_file("config.toml")?;
        Ok(app_config)
    }
    pub async fn to_state(&self)->Result<AppState,Box<dyn Error>>
    {
        let saver=self.data_saver_config.to_saver().await?;
        //todo add logginig
        let state=AppState::new(saver);
        Ok(state)
    }
}

impl ToRepositories for AppConfig {
    fn to_postgres_repo(&self) -> impl Future<Output = Result<PostgresRepo, Box<dyn Error>>> {
        async {
            let mut config = Config::new();
            config
                .user(self.postgres_config.user.as_str())
                .password(self.postgres_config.password.as_str())
                .dbname(self.postgres_config.db.as_str())
                .host(self.postgres_config.host.as_str())
                .port(self.postgres_config.port)
                .ssl_mode(Disable)
                .connect_timeout(Duration::from_secs(self.postgres_config.connect_timeout));
            let (client, connection) = config.connect(NoTls).await?;
            tokio::spawn(async move {
                if let Err(err) = connection.await {
                    println!("Error connection Posgres {}", err)
                }
            });

            let repo = PostgresRepo::new(client);
            Ok(repo)
        }
    }
}
