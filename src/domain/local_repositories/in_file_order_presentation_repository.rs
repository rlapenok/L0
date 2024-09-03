use std::collections::VecDeque;

use crate::{
    domain::remote_repositories::Destination, errors::local_repository_error::LocalRepositoryError,
};

pub type PostgresRawDataFromFile = Option<VecDeque<String>>;
pub type RedisRawDataInFromFile = Option<VecDeque<String>>;


pub trait InFileOrderPresentationRepository: Send + Sync + Clone {
    async fn save_orders(
        &self,
        data: Vec<String>,
        file: Destination,
    ) -> Result<(), LocalRepositoryError>;
    async fn get_raw_orders(
        &self,
    ) -> Result<(PostgresRawDataFromFile,PostgresRawDataFromFile), LocalRepositoryError>;
}
