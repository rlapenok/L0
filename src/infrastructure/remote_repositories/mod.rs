use deadpool_redis::Pool as RedisPool;
use postgres_order_repository::PostgresOrderRepository;
use redis_order_repository::RedisOrderRepository;
use sqlx::{postgres::PgRow, FromRow, Pool as PoolPostgres, Postgres};
use tracing::{debug, instrument, trace};

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
    #[instrument(
        skip(self, order, order_uid, value),
        name = "OrderPresentationRemoteRepository::save_order"
    )]
    async fn save_order<E: EntityForSave>(
        &self,
        order: &E,
        order_uid: &str,
        value: &str,
    ) -> Result<(), RemoteRepositoryError> {
        trace!("Start save order");
        //save in postgres
        self.postgres
            .save_order(order)
            .await
            .inspect_err(|err| trace!("Error when saving order in Postgres:{}", err))?;
        //save in redis
        if !self
            .redis
            .save_order(order_uid, value)
            .await
            .inspect_err(|err| trace!("Error when saving order in Redis:{}", err))?
        {
            return Err(RemoteRepositoryError::RedisUniqueErrorAndPosgresOk);
        }
        trace!("Order was saved");
        Ok(())
    }
    #[instrument(skip(self), name = "OrderPresentationRemoteRepository::get_order")]
    async fn get_order<T>(
        &self,
        order_uid: &str,
    ) -> Result<RemoteRepositoryResponse<T>, RemoteRepositoryError>
    where
        T: for<'row> FromRow<'row, PgRow> + Send + Unpin,
    {
        trace!("Start get orderd");
        //find in redis
        let redis_result = self.redis.get_order(order_uid).await;

        match redis_result {
            Ok(value) => {
                trace!("Order was received from Redis");
                Ok(RemoteRepositoryResponse::OrderFromRedis(value))
            }

            Err(err) => {
                trace!("Order was received from Redis with error:{}", err);
                let order = self
                    .postgres
                    .get_order::<T>(order_uid)
                    .await
                    .inspect_err(|err| {
                        trace!("Order was received from Postgres with error:{}", err)
                    })?;
                trace!("Order was received from Postgres with Redis Error:{}", err);
                Ok(RemoteRepositoryResponse::OrderFromPostgres(
                    order,
                    err.to_string(),
                ))
            }
        }
    }
    #[instrument(
        skip(self, order_uid, order),
        name = "OrderPresentationRemoteRepository::save_order_in_redis"
    )]
    async fn save_order_in_redis(
        &self,
        order_uid: &str,
        order: &str,
    ) -> Result<(), RemoteRepositoryError> {
        let result = self
            .redis
            .save_order(order, order_uid)
            .await
            .inspect_err(|err| {
                trace!("Order was received from Redis with error:{}", err);
            })?;
        if !result {
            return Err(RemoteRepositoryError::RedisUniqueErrorAndPosgresOk);
        }
        debug!("Order saved successfully ");
        Ok(())
    }
}
