use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, FromRow};

use crate::{
    domain::{
        models::EntityForSave, remote_repositories::OrderPresentationRemoteRepository,
        services::remote_order_presentation_remote_service::RemoteOrderRepresentationService,
    },
    errors::{
        remote_repository_error::RemoteRepositoryResponse, remote_service_error::RemoteServiceError,
    },
};

#[derive(Clone)]
pub struct RemoteService<R>
where
    R: OrderPresentationRemoteRepository + Send + Sync + Clone,
{
    repo: R,
}

impl<T> RemoteService<T>
where
    T: OrderPresentationRemoteRepository + Send + Sync + Clone,
{
    pub fn new(repo: T) -> Self {
        Self { repo }
    }
}


impl<R> RemoteOrderRepresentationService for RemoteService<R>
where
    R: OrderPresentationRemoteRepository + Send + Sync + Clone,
{
    async fn save_order<E: EntityForSave + Serialize>(
        &self,
        entity: &E,
    ) -> Result<(), RemoteServiceError> {
        let order_uid = entity.get_order_uid();
        let serialize_data = serde_json::to_string_pretty(entity)?;
        //save in remote repository
        self.repo
            .save_order(entity, order_uid, &serialize_data)
            .await?;
        Ok(())
    }
    async fn get_order<T>(&self, data_uid: String) -> Result<T, RemoteServiceError>
    where
        T: for<'de> Deserialize<'de> + for<'row> FromRow<'row, PgRow> + Send + Unpin,
    {
        let result = self.repo.get_order::<T>(data_uid).await?;
        match result {
            RemoteRepositoryResponse::OrderFromRedis(order) => {
                let order = serde_json::from_str::<T>(&order)?;
                Ok(order)
            }
            RemoteRepositoryResponse::OrderFromPostgres(order) => Ok(order),
        }
    }
}
