use deadpool_redis::Pool as RedisPool;
use postgres_order_repository::PostgresOrderRepository;
use redis_order_repository::RedisOrderRepository;
use sqlx::{postgres::PgRow, FromRow, Pool as PoolPostgres, Postgres};

use crate::{
    domain::{
        models::EntityForSave,
        remote_repositories::{
            postgres_order_presentation_repository::PostgresOrderPresentationRepository,
            redis_order_presentation_repository::RedisOrderPresentationRepository,
            OrderPresentationRemoteRepository,
        },
    },
    errors::remote_repository_error::{RemoteRepositoryError, RemoteRepositoryResponse},
};

pub(crate) mod postgres_order_repository;
pub(crate) mod redis_order_repository;

#[derive(Clone)]
pub struct RemoteRepository<P, R>
where
    P: PostgresOrderPresentationRepository,
    R: RedisOrderPresentationRepository,
{
    postgres: P,
    redis: R,
}

impl RemoteRepository<PostgresOrderRepository, RedisOrderRepository> {
    pub fn new(postgres_pool: PoolPostgres<Postgres>, redis_pool: RedisPool) -> Self {
        let postgres_repo = PostgresOrderRepository::new(postgres_pool);
        let redis_repo = RedisOrderRepository::new(redis_pool);
        Self {
            postgres: postgres_repo,
            redis: redis_repo,
        }
    }
}

impl OrderPresentationRemoteRepository
    for RemoteRepository<PostgresOrderRepository, RedisOrderRepository>
{
    async fn save_order<E: EntityForSave>(
        &self,
        entity: &E,
        key: &str,
        value: &str,
    ) -> Result<(), RemoteRepositoryError> {
        //save in postgres
        self.postgres.save_order(entity).await?;
        //save in redis
        if !self.redis.save_order(key, value).await? {
            return Err(RemoteRepositoryError::RedisUniqueErrorAndPosgresOk);
        }
        Ok(())
    }
    async fn get_order<T>(
        &self,
        data_uid: String,
    ) -> Result<RemoteRepositoryResponse<T>, RemoteRepositoryError>
    where
        T: for<'row> FromRow<'row, PgRow> + Send + Unpin,
    {
        //find in redis
        let redis_result = self.redis.get_order(&data_uid).await;

        match redis_result {
            Ok(value) => Ok(RemoteRepositoryResponse::OrderFromRedis(value)),

            Err(_) => {
                let order = self.postgres.get_order::<T>(&data_uid).await?;
                Ok(RemoteRepositoryResponse::OrderFromPostgres(order))
            }
        }
    }
}
