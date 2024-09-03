use std::error::Error;

use crate::{
    domain::local_repositories::in_memory_order_presentation_repository::{
        Entity, PostgresRawDataInMemory, RedisRawDataInMemory}
    ,
    errors::local_service_error::LocalServiceErrors,
};

pub trait LocalOrderRepresentationService {
    async fn read_raw_orders_from_file_and_save_in_memory(&self) -> Result<(), LocalServiceErrors>;
    fn get_postrges_from_memoty(&self) -> PostgresRawDataInMemory;
    async fn save_postgres_in_memory(&self, order: Box<Entity>);
    fn get_redis_from_memory(&self) -> RedisRawDataInMemory;

    async fn save_redis_in_memory(&self, order: Box<Entity>);
}

pub trait ToLocalOrderRepresentationService<T>
where
    T: LocalOrderRepresentationService,
{
    async fn to_local_service(&self) -> Result<T, Box<dyn Error>>;
}
