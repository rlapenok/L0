use file_repository::FileRepository;
use in_memory_repository::InMemoryReposiory;
use std::collections::VecDeque;
use tokio::fs::File;
use tracing::{debug, instrument};

use crate::{
    domain::{
        local_repositories::{
            in_file_order_presentation_repository::{
                InFileOrderPresentationRepository, PostgresRawDataFromFile, RedisRawDataInFromFile,
            },
            in_memory_order_presentation_repository::{
                Entity, InMemoryOrderPresentationRepository, RawOrdersInMemory,
            },
            OrderPresentationLocalRepository,
        },
        models::Destination,
    },
    errors::local_repository_error::LocalRepositoryError,
};

pub mod file_repository;
pub mod in_memory_repository;

#[derive(Clone)]
pub struct LocalRepository {
    in_file: FileRepository,
    in_memory: InMemoryReposiory,
}

impl LocalRepository {
    pub fn new(files: (File, File)) -> Self {
        let in_file = FileRepository::new(files);
        let in_memory = InMemoryReposiory::new();
        Self { in_file, in_memory }
    }
}

impl OrderPresentationLocalRepository for LocalRepository {
    #[instrument(
        skip(self),
        name = "OrderPresentationLocalRepository::read_raw_orders_from_files"
    )]
    async fn read_raw_orders_from_files(
        &self,
    ) -> Result<(PostgresRawDataFromFile, RedisRawDataInFromFile), LocalRepositoryError> {
        debug!("Starting to read raw orders from files");
        let raw_orders = self.in_file.get_raw_orders().await.inspect_err(|err| {
            debug!("Error when reading orders from files:{}", err);
        })?;
        debug!("Orders were successfully read from files");
        Ok(raw_orders)
    }
    #[instrument(
        skip(self, orders),
        name = "OrderPresentationLocalRepository::save_orders_in_memory"
    )]
    async fn save_orders_in_memory(&self, orders: (VecDeque<Box<Entity>>, VecDeque<Box<Entity>>)) {
        self.in_memory.save_raw_orders(orders).await;
    }
    fn get_raw_orders_from_memory(&self, dest: Destination) -> RawOrdersInMemory {
        self.in_memory.get_raw_orders(dest)
    }
    async fn save_row_orders_in_file(&self, orders: VecDeque<String>, dest: Destination) {
        self.in_file.save_orders(orders, dest).await;
    }
    async fn save_in_memory(&self, dest: Destination, order: Box<Entity>) {
        self.in_memory.save_raw_order(dest, order).await
    }
}
