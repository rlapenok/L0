use crate::domain::{
    local_repositories::in_memory_order_presentation_repository::{
        Entity, InMemoryOrderPresentationRepository, RawOrdersInMemory,
    },
    models::Destination,
};
use std::{collections::VecDeque, sync::Arc};

use tokio::sync::{Mutex, MutexGuard};
use tracing::{instrument, trace};

#[derive(Clone)]
pub struct InMemoryReposiory {
    postgres_raw_data: RawOrdersInMemory,
    redis_raw_data: RawOrdersInMemory,
}
impl InMemoryReposiory {
    pub(crate) fn new() -> Self {
        Self {
            postgres_raw_data: Arc::new(Mutex::new(VecDeque::new())),
            redis_raw_data: Arc::new(Mutex::new(VecDeque::new())),
        }
    }
    //method for get guard
    async fn get_guard(&self, dest: Destination) -> MutexGuard<'_, VecDeque<Box<Entity>>> {
        match dest {
            Destination::Postgres => self.postgres_raw_data.lock().await,
            Destination::Redis => self.redis_raw_data.lock().await,
        }
    }
    //method for save
    async fn save(&self, dest: Destination, order: Box<Entity>) {
        let mut guard = self.get_guard(dest).await;
        guard.push_back(order)
    }
}
impl InMemoryOrderPresentationRepository for InMemoryReposiory {
    #[instrument(
        skip(self, orders),
        name = "InMemoryOrderPresentationRepository::save_raw_orders"
    )]
    async fn save_raw_orders(&self, orders: (VecDeque<Box<Entity>>, VecDeque<Box<Entity>>)) {
        trace!("Start save raw orders");
        let mut postgres_guard = self.get_guard(Destination::Postgres).await;
        *postgres_guard = orders.0;
        let mut redis_guard = self.get_guard(Destination::Redis).await;
        *redis_guard = orders.1;
        trace!("Raw order was saved")
    }
    #[instrument(skip(self, dest, order))]
    async fn save_raw_order(&self, dest: Destination, order: Box<Entity>) {
        trace!("Start save raw order");
        self.save(dest, order).await;
        trace!("Raw order was saved")
    }
    fn get_raw_orders(&self, dest: Destination) -> RawOrdersInMemory {
        match dest {
            Destination::Postgres => self.postgres_raw_data.clone(),
            _ => self.redis_raw_data.clone(),
        }
    }
}
