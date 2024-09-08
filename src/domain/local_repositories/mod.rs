use std::collections::VecDeque;

use in_file_order_presentation_repository::{PostgresRawDataFromFile, RedisRawDataInFromFile};
use in_memory_order_presentation_repository::{Entity, RawOrdersInMemory};

use crate::errors::local_repository_error::LocalRepositoryError;

use super::models::Destination;

pub mod in_file_order_presentation_repository;
pub mod in_memory_order_presentation_repository;

//trait for generalizing methods for get/save raw orders from file/memory
#[allow(async_fn_in_trait)]
pub trait OrderPresentationLocalRepository: Send + Sync + Clone {
    async fn read_raw_orders_from_files(
        &self,
    ) -> Result<(PostgresRawDataFromFile, RedisRawDataInFromFile), LocalRepositoryError>;
    async fn save_orders_in_memory(&self, orders: (VecDeque<Box<Entity>>, VecDeque<Box<Entity>>));
    async fn save_row_orders_in_file(&self, orders: VecDeque<String>, dest: Destination);
    fn get_raw_orders_from_memory(&self, dest: Destination) -> RawOrdersInMemory;
    async fn save_in_memory(&self, dest: Destination, order: Box<Entity>);
}
