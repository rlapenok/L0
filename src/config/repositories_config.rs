use std::{error::Error, time::Duration};

use confique::Config;
use deadpool_redis::{
    Config as RedisPoolConfig, ConnectionAddr, ConnectionInfo, Pool as RedisPool,
    RedisConnectionInfo, Runtime::Tokio1,
};

use sqlx::{pool::PoolOptions, postgres::PgConnectOptions, Pool, Postgres};
use tokio::fs::File;

use crate::{
    domain::services::{
        local_order_presentation_remote_services::ToLocalOrderRepresentationService,
        remote_order_presentation_remote_service::ToRemoteOrderRepresentationService,
    },
    infrastructure::{
        local_repositories::LocalRepository,
        remote_repositories::RemoteRepository,
        services::{order_presentation_remote_service::RemoteService, LocalSrv, RemoteSrv},
    },
};

#[derive(Config, Debug)]
pub struct RepositoriesConfig {
    #[config(nested)]
    postgres_config: PostgresRepoConfig,
    #[config(nested)]
    redis_config: RedisRepoConfig,
    #[config(nested)]
    local_config: LocalRepoConfig,
}
#[derive(Config, Debug)]
pub struct LocalRepoConfig {
    path1: String,
    path2: String,
}

impl LocalRepoConfig {
    async fn get_files(&self) -> Result<(File, File), Box<dyn Error>> {
        let postgres_file = File::options()
            .append(true)
            .read(true)
            .create(true)
            .open(&self.path1)
            .await?;
        let redis_file = File::options()
            .append(true)
            .read(true)
            .create(true)
            .open(&self.path2)
            .await?;
        Ok((postgres_file, redis_file))
    }
}

#[derive(Config, Debug)]
pub struct PostgresRepoConfig {
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
    fn to_connect_opt(&self) -> PgConnectOptions {
        PgConnectOptions::new()
            .host(&self.host)
            .port(self.port)
            .database(&self.db)
            .username(&self.login)
            .password(&self.password)
    }
    fn to_pool_opt(&self) -> PoolOptions<Postgres> {
        PoolOptions::new()
            .max_lifetime(Some(Duration::from_secs(self.max_lifetime_sec)))
            .idle_timeout(Some(Duration::from_secs(self.idle_timeout_sec)))
            .max_connections(self.max_connections)
            .test_before_acquire(true)
            //todo Setup in config
            .acquire_timeout(Duration::from_secs(3))
    }
    pub async fn create_pool(&self) -> Result<Pool<Postgres>, sqlx::Error> {
        let pool_opt = self.to_pool_opt();
        let conn_opt = self.to_connect_opt();
        pool_opt.connect_with(conn_opt).await
    }
}

#[derive(Config, Debug)]
pub struct RedisRepoConfig {
    host: String,
    port: u16,
    password: String,
    db: i64,
    max_connections: usize,
    wait: u64,
    create: u64,
}

impl RedisRepoConfig {
    async fn create_pool(&self) -> Result<RedisPool, Box<dyn Error>> {
        let connection_address = ConnectionAddr::Tcp(self.host.clone(), self.port);
        RedisConnectionInfo::default();
        let redis_connection_info = RedisConnectionInfo {
            db: self.db,
            password: Some(self.password.clone()),
            ..Default::default()
        };
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

impl ToRemoteOrderRepresentationService<RemoteSrv> for RepositoriesConfig {
    async fn to_remote_service(&self) -> Result<RemoteSrv, Box<dyn Error>> {
        let postgres_pool = self.postgres_config.create_pool().await?;
        let redis_pool = self.redis_config.create_pool().await?;
        let repository = RemoteRepository::new(postgres_pool, redis_pool);
        Ok(RemoteService::new(repository))
    }
}

impl ToLocalOrderRepresentationService<LocalSrv> for RepositoriesConfig {
    async fn to_local_service(&self) -> Result<LocalSrv, Box<dyn Error>> {
        let files = self.local_config.get_files().await?;
        let repository = LocalRepository::new(files);
        Ok(LocalSrv::new(repository))
    }
}
