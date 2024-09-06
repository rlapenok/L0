use std::collections::VecDeque;

use crate::{domain::models::Destination, errors::local_repository_error::LocalRepositoryError};

pub type PostgresRawDataFromFile = Option<VecDeque<String>>;
pub type RedisRawDataInFromFile = Option<VecDeque<String>>;

//trait for save and get raw in files when server up and shutdown
pub trait InFileOrderPresentationRepository: Send + Sync + Clone {
    //method for save order when server graceful shutdown
    async fn save_orders(&self, raw_orders: VecDeque<String>, file: Destination);
    //get raw orders when server up
    async fn get_raw_orders(
        &self,
    ) -> Result<(PostgresRawDataFromFile, PostgresRawDataFromFile), LocalRepositoryError>;
}
