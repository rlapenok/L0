//trait for business logic to save/get order in/from Redis
#[allow(async_fn_in_trait)]

pub trait RedisOrderPresentationRepository: Send + Sync + Clone {
    //method for save order
    async fn save_order(&self, key: &str, value: &str) -> Result<bool, deadpool_redis::PoolError>;
    //method for get order on order_uid
    async fn get_order(&self, order_uid: &str) -> Result<String, deadpool_redis::PoolError>;
}
