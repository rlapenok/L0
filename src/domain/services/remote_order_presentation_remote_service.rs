use std::error::Error;

use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, FromRow};

use crate::{domain::models::EntityForSave, errors::remote_service_error::RemoteServiceError};

pub trait ToRemoteOrderRepresentationService<T>
where
    T: RemoteOrderRepresentationService,
{
    async fn to_remote_service(self) -> Result<T, Box<dyn Error>>;
}

pub trait RemoteOrderRepresentationService: Send + Clone {
    async fn save_order<E: EntityForSave + Serialize>(
        &self,
        data: &E,
    ) -> Result<(), RemoteServiceError>;
    async fn get_order<T>(&self, data_uid: String) -> Result<T, RemoteServiceError>
    where
        T: for<'de> Deserialize<'de> + for<'row> FromRow<'row, PgRow> + Send + Unpin;
}
