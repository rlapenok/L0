use deadpool_redis::{redis::cmd, Pool, PoolError};
use redis::{AsyncCommands, FromRedisValue};
use tracing::{instrument, trace};

use crate::domain::remote_repositories::redis_order_presentation_repository::RedisOrderPresentationRepository;

#[derive(Clone)]
pub struct RedisOrderRepository(Pool);

impl RedisOrderRepository {
    pub(crate) fn new(pool: Pool) -> Self {
        Self(pool)
    }
}

impl RedisOrderPresentationRepository for RedisOrderRepository {
    #[instrument(
        skip(self, order_uid, order),
        name = "RedisOrderPresentationRepository::save_order"
    )]
    async fn save_order(
        &self,
        order_uid: &str,
        order: &str,
    ) -> Result<bool, deadpool_redis::PoolError> {
        trace!("Start save order");
        let mut connection = self.0.get().await.inspect_err(|err| {
            trace!("🚨Error receiving connection:{}", err);
        })?;
        let cmd_start = cmd("MULTI");
        let cmd_end = cmd("EXEC");
        connection
            .send_packed_command(&cmd_start)
            .await
            .map_err(PoolError::from)
            .inspect_err(|err| {
                trace!("🚨Error starting transaction:{}", err);
            })?;
        connection
            .set_nx(order_uid, order)
            .await
            .map_err(PoolError::from)
            .inspect_err(|err| {
                trace!("🚨Error during transaction:{}", err);
            })?;
        let result = connection
            .send_packed_command(&cmd_end)
            .await
            .map_err(PoolError::from)
            .inspect_err(|err| {
                trace!("🚨Error commiting transaction:{}", err);
            })?;
        let result = Vec::<u8>::from_redis_value(&result).inspect_err(|err| {
            trace!("🚨Error transforming response:{}", err);
        })?;
        let result = result[0];
        if result == 0 {
            trace!("🚨 This order_uid exists in Radis");
            return Ok(false);
        }
        trace!("Order was saved");
        Ok(true)
    }
    #[instrument(
        skip(self, order_uid),
        name = "RedisOrderPresentationRepository::get_order"
    )]
    async fn get_order(&self, order_uid: &str) -> Result<String, deadpool_redis::PoolError> {
        trace!("Start get order");
        let mut connection = self.0.get().await?;
        let cmd_start = cmd("MULTI");
        let cmd_end = cmd("EXEC");
        connection
            .send_packed_command(&cmd_start)
            .await
            .map_err(PoolError::from)
            .inspect_err(|err| {
                trace!("🚨Error starting transaction:{}", err);
            })?;
        connection
            .get(order_uid)
            .await
            .map_err(PoolError::from)
            .inspect_err(|err| {
                trace!("🚨Error during transaction:{}", err);
            })?;

        let value = connection
            .send_packed_command(&cmd_end)
            .await
            .map_err(PoolError::from)
            .inspect_err(|err| {
                trace!("🚨Error commiting transaction:{}", err);
            })?;
        let order = Vec::<String>::from_redis_value(&value)
            .inspect_err(|err| trace!("🚨Error transforming response:{}", err))?;
        trace!("Order was get");
        Ok(order[0].clone())
    }
}
