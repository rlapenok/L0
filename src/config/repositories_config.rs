use std::{error::Error, future::Future, time::Duration};

use confique::Config;
use sqlx::{pool::PoolOptions, postgres::PgConnectOptions};

use crate::contracts::repositories::ToRepositories;
use crate::repo::repository::{Repository, RepositoryType};

#[derive(Config)]
pub(crate) struct RepositoriesConfig {
    #[config(nested)]
    postgres_config: PostgresRepoConfig,
    #[config(nested)]
    redis_config: RedisRepoConfig,
}

#[derive(Config)]
struct PostgresRepoConfig {
    host: String,
    port: u16,
    login: String,
    password: String,
    db: String,
    max_connections: u32,
    idle_timeout_sec: u64,
    max_lifetime_sec: u64,
}

#[derive(Config)]
struct RedisRepoConfig {}

impl ToRepositories for RepositoriesConfig {
    type Output = RepositoryType;
    fn to_repositories(
        &self,
    ) -> impl Future<Output = Result<Self::Output, Box<dyn Error>>> {
        let connect_opts = PgConnectOptions::new()
            .host(&self.postgres_config.host)
            .port(self.postgres_config.port)
            .database(&self.postgres_config.db)
            .username(&self.postgres_config.login)
            .password(&self.postgres_config.password);
        let pool_opts = PoolOptions::new()
            .max_lifetime(Some(Duration::from_secs(
                self.postgres_config.max_lifetime_sec,
            )))
            .idle_timeout(Some(Duration::from_secs(
                self.postgres_config.idle_timeout_sec,
            )))
            .max_connections(self.postgres_config.max_connections)
            .test_before_acquire(true)
            //todo Setup in config
            .acquire_timeout(Duration::from_secs(3));
        async {
            let postgres_pool = pool_opts.connect_with(connect_opts).await?;
            let redis_pool = ();
            let repo = Repository::new(postgres_pool, redis_pool);
            Ok(repo)
        }
    }
}
