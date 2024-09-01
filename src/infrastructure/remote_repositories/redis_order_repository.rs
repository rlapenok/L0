use deadpool_redis::{redis::cmd, Pool, PoolError};
use redis::AsyncCommands;

use crate::{
    domain::remote_repositories::redis_order_presentation_repository::RedisOrderPresentationRepository,
    errors::remote_repository_error::RemoteRepositoryError,
};

#[derive(Clone)]
pub struct RedisOrderRepository(Pool);

impl RedisOrderRepository {
    pub(crate) fn new(pool: Pool) -> Self {
        Self(pool)
    }
}

impl RedisOrderPresentationRepository for RedisOrderRepository {
    async fn save_order(&self, key: &str, value: &str) -> Result<(), RemoteRepositoryError> {
        let mut connection = self.0.get().await?;
        let cmd_start = cmd("MULTI");
        let cmd_end = cmd("EXEC");
        connection
            .send_packed_command(&cmd_start)
            .await
            .map_err(|err| PoolError::from(err))?;
        let _: () = connection
            .set(key, value)
            .await
            .map_err(|err| PoolError::from(err))?;
        connection
            .send_packed_command(&cmd_end)
            .await
            .map_err(|err| PoolError::from(err))?;
        Ok(())
    }
    async fn get_order(&self, data_uid: &str) ->Result<String,RemoteRepositoryError>{

        let mut connection = self.0.get().await?;
        let cmd_start = cmd("MULTI");
        let cmd_end = cmd("EXEC");
        connection
        .send_packed_command(&cmd_start)
        .await
        .map_err(|err| PoolError::from(err))?;
        let result:String=connection.get(data_uid).await.map_err(|err|{
            PoolError::from(err)
        })?;
        connection
            .send_packed_command(&cmd_end)
            .await
            .map_err(|err| PoolError::from(err))?;
        Ok(result)
    }
}
