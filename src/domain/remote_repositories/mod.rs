use std::error::Error;

use sqlx::{postgres::PgRow, FromRow};

use crate::errors::remote_repository_error::RemoteRepositoryError;

use super::models::EntityForSave;

pub mod postgres_order_presentation_repository;
pub mod redis_order_presentation_repository;

#[derive(Clone)]
pub enum Destination {
    PostgresFile,
    RedisFile,
}

//trait for converting some entity to OrderPresentationRepository
pub trait ToOrderPresentationRepository<R>
where
    R: OrderPresentationRemoteRepository,
{
    async fn to_repository(self) -> Result<R, Box<dyn Error>>;
}

pub trait OrderPresentationRemoteRepository {
    async fn save_order<E: EntityForSave>(
        &self,
        entity: &E,
        key: &str,
        value: &str,
    ) -> Result<(), RemoteRepositoryError>;
    async fn get_order<T>(&self, order_uid: String)->Result<T,RemoteRepositoryError>
    where T: for<'a> FromRow<'a, PgRow>+Send+Unpin;

}
