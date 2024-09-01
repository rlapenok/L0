//trait for business logic to save/return orders in Redis
use crate::errors::remote_repository_error::RemoteRepositoryError;

pub trait RedisOrderPresentationRepository:Send+Sync+Clone {
    async fn save_order(&self, key: &str, value: &str) -> Result<(), RemoteRepositoryError>;
    async fn get_order(&self, data_uid: &str)->Result<String,RemoteRepositoryError>;
}
