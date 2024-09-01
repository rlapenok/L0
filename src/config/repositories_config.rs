use std::{error::Error, time::Duration};

use crate::{
    domain::remote_repositories::ToOrderPresentationRepository,
    infrastructure::{
        remote_repositories::OrderPresentationRemoteRepository, services::RemoteRepository,
    },
};
use confique::Config;
use deadpool_redis::{
    Config as RedisPoolConfig, ConnectionAddr, ConnectionInfo, Pool as RedisPool,
    RedisConnectionInfo, Runtime::Tokio1,
};
use sqlx::{pool::PoolOptions, postgres::PgConnectOptions, Postgres};
use tokio::fs::File;

#[derive(Config, Debug)]
pub(crate) struct RepositoriesConfig {
    #[config(nested)]
    postgres_config: PostgresRepoConfig,
    #[config(nested)]
    redis_config: RedisRepoConfig,
    #[config(nested)]
    data_saver_writer_config: DataSaverWriterConfig,
}
#[derive(Config, Debug)]
pub struct DataSaverWriterConfig {
    path1: String,
    path2: String,
}

impl DataSaverWriterConfig {
    async fn get_files(&self) -> Result<(File, File), Box<dyn Error>> {
        let postgres_file = File::options()
            .append(true)
            .read(true)
            .create(true)
            .open(&self.path1)
            .await
            .unwrap();
        let redis_file = File::options()
            .append(true)
            .read(true)
            .create(true)
            .open(&self.path2)
            .await
            .unwrap();
        Ok((postgres_file, redis_file))
    }
}

#[derive(Config, Debug)]
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

impl PostgresRepoConfig {
    async fn to_connect_opt(&self) -> PgConnectOptions {
        PgConnectOptions::new()
            .host(&self.host)
            .port(self.port)
            .database(&self.db)
            .username(&self.login)
            .password(&self.password)
    }
    async fn to_pool_opt(&self) -> PoolOptions<Postgres> {
        PoolOptions::new()
            .max_lifetime(Some(Duration::from_secs(self.max_lifetime_sec)))
            .idle_timeout(Some(Duration::from_secs(self.idle_timeout_sec)))
            .max_connections(self.max_connections)
            .test_before_acquire(true)
            //todo Setup in config
            .acquire_timeout(Duration::from_secs(3))
    }
}

#[derive(Config, Debug)]
struct RedisRepoConfig {
    host: String,
    port: u16,
    login: String,
    password: String,
    db: i64,
    max_connections: usize,
    wait: u64,
    create: u64,
}

impl RedisRepoConfig {
    async fn create_pool(self) -> Result<RedisPool, Box<dyn Error>> {
        let connection_address = ConnectionAddr::Tcp(self.host, self.port);
        let mut redis_connection_info = RedisConnectionInfo::default();
        redis_connection_info.db = self.db;
        // redis_connection_info.username = Some(self.login);
        redis_connection_info.password = Some(self.password);
        let connection_info = ConnectionInfo {
            addr: connection_address,
            redis: redis_connection_info,
        };
        let config = RedisPoolConfig::from_connection_info(connection_info);
        let pool = config
            .builder()?
            .max_size(self.max_connections)
            .wait_timeout(Some(Duration::from_secs(self.wait)))
            .create_timeout(Some(Duration::from_secs(self.create)))
            .runtime(Tokio1)
            .recycle_timeout(None)
            .build()?;
        pool.get().await?;
        Ok(pool)
    }
}

//convert RepositoriesConfig to OrderPresentationRepository trait
impl ToOrderPresentationRepository<RemoteRepository> for RepositoriesConfig {
    async fn to_repository(self) -> Result<RemoteRepository, Box<dyn Error>> {
        let connect_opt = self.postgres_config.to_connect_opt().await;
        let pool_opt = self.postgres_config.to_pool_opt().await;
        let files = self.data_saver_writer_config.get_files().await?;
        let postgres_pool = pool_opt.connect_with(connect_opt).await?;
        let redis_pool = self.redis_config.create_pool().await?;
        let repo = OrderPresentationRemoteRepository::new(postgres_pool, redis_pool, files);
        Ok(repo)
    }
}
