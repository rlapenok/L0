
//trait for business logic to save/return orders in Redis
pub trait RedisOrderPresentationRepository: Send + Sync + Clone {
    async fn save_order(&self, key: &str, value: &str) -> Result<bool, deadpool_redis::PoolError>;
    async fn get_order(&self, data_uid: &str) -> Result<String, deadpool_redis::PoolError>;
}
