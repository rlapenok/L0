use std::collections::VecDeque;

use file_repository::FileRepository;
use in_memory_repository::InMemoryReposiory;
use tokio::fs::File;

use crate::{
    domain::local_repositories::{
        in_file_order_presentation_repository::{InFileOrderPresentationRepository, PostgresRawDataFromFile, RedisRawDataInFromFile},
        in_memory_order_presentation_repository::{
            Entity, InMemoryOrderPresentationRepository, PostgresRawDataInMemory, RedisRawDataInMemory
        },
        OrderPresentationLocalRepository,
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

    async fn read_raw_orders_from_files(&self)->Result<(PostgresRawDataFromFile,RedisRawDataInFromFile),LocalRepositoryError> {
        let raw_orders=self.in_file.get_raw_orders().await?;
        Ok(raw_orders)
    }

    async fn save_orders_in_memory(&self,orders:(VecDeque<Box<Entity>>,VecDeque<Box<Entity>>)){
            self.in_memory.save_raw_orders(orders).await;
    }
    fn get_postgres_from_memory(&self) -> PostgresRawDataInMemory {
        self.in_memory.get_postgres_from_memory()
    }
    async fn save_postgres_in_memory(&self, order: Box<Entity>) {
        self.in_memory.save_postgres(order).await
    }
    fn get_redis_from_memory(&self) -> RedisRawDataInMemory {
        self.in_memory.get_redis_from_memory()
    }
    async fn save_redis_in_memory(&self, order: Box<Entity>) {
        self.in_memory.save_redis(order).await
    }
}
