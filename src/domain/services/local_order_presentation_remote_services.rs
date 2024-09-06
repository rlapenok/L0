use std::{collections::VecDeque, error::Error};

use crate::{
    domain::{
        local_repositories::in_memory_order_presentation_repository::{Entity, RawOrdersInMemory},
        models::Destination,
    },
    errors::local_service_error::LocalServiceErrors,
};

//trait for implementing a service for save/get raw orders from local repository
pub trait LocalOrderRepresentationService {
    //method for read raw orders from files and save in memory
    async fn read_raw_orders_from_file_and_save_in_memory(&self) -> Result<(), LocalServiceErrors>;
    //method for save raw order in file when server graceful shutdown
    async fn save_raw_orders_in_file(&self, orders: &mut VecDeque<Box<Entity>>, dest: Destination);
    //method for save order in memory(use in background tasks and when server up)
    async fn save_in_memory(&self, dest: Destination, order: Box<Entity>);
    //method for get raw order (use in background task)
    fn get_raw_orders_from_memory(&self, dest: Destination) -> RawOrdersInMemory;
}

//trait for convert entity to LocalOrderRepresentationService
pub trait ToLocalOrderRepresentationService<T>
where
    T: LocalOrderRepresentationService,
{
    async fn to_local_service(&self) -> Result<T, Box<dyn Error>>;
}
