use std::collections::VecDeque;

use in_file_order_presentation_repository::{PostgresRawDataFromFile, RedisRawDataInFromFile};
use in_memory_order_presentation_repository::{Entity, PostgresRawDataInMemory, RedisRawDataInMemory};

use crate::errors::local_repository_error::LocalRepositoryError;

pub mod in_file_order_presentation_repository;
pub mod in_memory_order_presentation_repository;

pub trait OrderPresentationLocalRepository: Send + Sync + Clone {
    async fn read_raw_orders_from_files(&self)->Result<(PostgresRawDataFromFile,RedisRawDataInFromFile),LocalRepositoryError>;
    async fn save_orders_in_memory(&self,orders:(VecDeque<Box<Entity>>,VecDeque<Box<Entity>>));
    fn get_postgres_from_memory(&self) -> PostgresRawDataInMemory;
    async fn save_postgres_in_memory(&self, order: Box<Entity>);
    fn get_redis_from_memory(&self) -> RedisRawDataInMemory;
    async fn save_redis_in_memory(&self, order: Box<Entity>);
}
