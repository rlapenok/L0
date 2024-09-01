
use crate::{domain::{models::EntityForSave, remote_repositories::Destination}, errors::local_repository_error::LocalRepositoryError};

pub trait LocalOrderRepresentationService {
    async fn save_in_memory<E:EntityForSave>(&self,data:E);
    async fn save_in_file(&self, data: String, file: Destination) -> Result<(), LocalRepositoryError>;
}
