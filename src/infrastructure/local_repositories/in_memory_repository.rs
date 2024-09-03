use crate::domain::{
    local_repositories::in_memory_order_presentation_repository::{
        Entity, InMemoryOrderPresentationRepository, PostgresRawDataInMemory, RedisRawDataInMemory
    },
    remote_repositories::Destination,
};
use std::{collections::VecDeque, mem::replace, sync::Arc};

use tokio::sync::{Mutex, MutexGuard};

#[derive(Clone)]
pub struct InMemoryReposiory {
    postgres_row_data: PostgresRawDataInMemory,
    redis_row_data: PostgresRawDataInMemory,
}
impl InMemoryReposiory {
    pub(crate) fn new() -> Self {
        Self {
            postgres_row_data: Arc::new(Mutex::new(VecDeque::new())),
            redis_row_data: Arc::new(Mutex::new(VecDeque::new())),
        }
    }
    async fn get_guard(&self, dest: Destination) -> MutexGuard<'_, VecDeque<Box<Entity>>> {
        match dest {
            Destination::PostgresFile => self.postgres_row_data.lock().await,
            Destination::RedisFile => self.redis_row_data.lock().await,
        }
    }
    async fn save(&self, dest: Destination, order: Box<Entity>) {
        let mut guard = self.get_guard(dest).await;
        guard.push_back(order)
    }
}
impl InMemoryOrderPresentationRepository for InMemoryReposiory {

    async fn save_raw_orders(&self, orders: (VecDeque<Box<Entity>>,VecDeque<Box<Entity>>)) {
        
        
        let mut postgres_guard = self.get_guard(Destination::PostgresFile).await;
        *postgres_guard=orders.0;
        let mut redis_guard = self.get_guard(Destination::RedisFile).await;
        *redis_guard = orders.1
        }
    async fn get_raw_orders(self) -> (VecDeque<Box<Entity>>, VecDeque<Box<Entity>>) {
        todo!()
    }
    fn get_postgres_from_memory(&self) -> PostgresRawDataInMemory {
        self.postgres_row_data.clone()
    }
    async fn save_postgres(&self, order: Box<Entity>) {
        self.save(Destination::PostgresFile, order).await
    }
    fn get_redis_from_memory(&self) -> RedisRawDataInMemory {
        self.redis_row_data.clone()
    }
    async fn save_redis(&self, order: Box<Entity>) {
        self.save(Destination::RedisFile, order).await
    }

}
