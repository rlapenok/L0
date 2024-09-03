
use sqlx::{postgres::PgRow, FromRow};

use crate::errors::remote_repository_error::{RemoteRepositoryError, RemoteRepositoryResponse};

use super::models::EntityForSave;

pub mod postgres_order_presentation_repository;
pub mod redis_order_presentation_repository;

#[derive(Clone)]
pub enum Destination {
    PostgresFile,
    RedisFile,
}

pub trait OrderPresentationRemoteRepository {
    async fn save_order<E: EntityForSave>(
        &self,
        entity: &E,
        key: &str,
        value: &str,
    ) -> Result<(), RemoteRepositoryError>;
    async fn get_order<T>(
        &self,
        order_uid: String,
    ) -> Result<RemoteRepositoryResponse<T>, RemoteRepositoryError>
    where
        T: for<'row> FromRow<'row, PgRow> + Send + Unpin;
}
