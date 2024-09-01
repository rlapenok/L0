use crate::{domain::remote_repositories::Destination, errors::local_repository_error::LocalRepositoryError};

pub trait InFileOrderPresentationRepository:Send+Sync+Clone {
    async fn save_order(&self, data: Vec<String>, dest: Destination) -> Result<(), LocalRepositoryError>;
    async fn get_row_data(&self, dest: Destination)-> Result<Option<Vec<String>>, LocalRepositoryError>;
}