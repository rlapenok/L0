use deadpool_redis::{redis::cmd, Pool, PoolError};
use redis::{from_redis_value, AsyncCommands, FromRedisValue, Value};

use crate::domain::remote_repositories::redis_order_presentation_repository::RedisOrderPresentationRepository;

#[derive(Clone)]
pub struct RedisOrderRepository(Pool);

impl RedisOrderRepository {
    pub(crate) fn new(pool: Pool) -> Self {
        Self(pool)
    }
}

impl RedisOrderPresentationRepository for RedisOrderRepository {
    async fn save_order(&self, key: &str, value: &str) -> Result<bool, deadpool_redis::PoolError> {
        let mut connection = self.0.get().await?;
        let cmd_start = cmd("MULTI");
        let cmd_end = cmd("EXEC");
        connection
            .send_packed_command(&cmd_start)
            .await
            .map_err(|err| PoolError::from(err))?;
        connection
            .set_nx(key, value)
            .await
            .map_err(|err| PoolError::from(err))?;
        let result = connection
            .send_packed_command(&cmd_end)
            .await
            .map_err(|err| PoolError::from(err))?;
        let result = Vec::<u8>::from_redis_value(&result)?;
        let result = result[0];
        if result == 0 {
            return Ok(false);
        }
        Ok(true)
    }
    async fn get_order(&self, data_uid: &str) -> Result<String, deadpool_redis::PoolError> {
        let mut connection = self.0.get().await?;
        let cmd_start = cmd("MULTI");
        let cmd_end = cmd("EXEC");
        connection
            .send_packed_command(&cmd_start)
            .await
            .map_err(|err| PoolError::from(err))?;
        connection
            .get(data_uid)
            .await
            .map_err(|err| PoolError::from(err))?;

        let value = connection
            .send_packed_command(&cmd_end)
            .await
            .map_err(|err| PoolError::from(err))?;
        println!("{:?}",value);
        let order = Vec::<String>::from_redis_value(&value)?;
        //println!("{:?}",order);
        Ok(
            "asdasds".to_owned()
            //order[0].clone()
            )
    }
}
