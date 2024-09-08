use std::error::Error;

use crate::{domain::models::EntityForSave, errors::remote_service_error::RemoteServiceError};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, FromRow};

//trait for implementing a service for save/get raw orders from remote repository
#[allow(async_fn_in_trait)]
pub trait RemoteOrderRepresentationService: Send + Clone {
    async fn save_order<E: EntityForSave + Serialize>(
        &self,
        order: &E,
    ) -> Result<(), RemoteServiceError>;
    async fn get_order<T>(
        &self,
        order_uid: &str,
    ) -> Result<(T, Option<String>), RemoteServiceError>
    where
        T: for<'de> Deserialize<'de> + for<'row> FromRow<'row, PgRow> + Send + Unpin;
    async fn save_order_in_redis<E: EntityForSave + Serialize>(
        &self,
        order: &E,
    ) -> Result<(), RemoteServiceError>;
}

//trait for convert entity to RemoteOrderRepresentationService
#[allow(async_fn_in_trait)]
pub trait ToRemoteOrderRepresentationService<T>
where
    T: RemoteOrderRepresentationService,
{
    async fn to_remote_service(&self) -> Result<T, Box<dyn Error>>;
}
