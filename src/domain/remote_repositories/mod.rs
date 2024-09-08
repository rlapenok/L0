use sqlx::{postgres::PgRow, FromRow};

use crate::errors::remote_repository_error::{RemoteRepositoryError, RemoteRepositoryResponse};

use super::models::EntityForSave;

pub mod postgres_order_presentation_repository;
pub mod redis_order_presentation_repository;

//trait for generalizing methods for save/get order from remote repository
#[allow(async_fn_in_trait)]
pub trait OrderPresentationRemoteRepository {
    //method for save order in Postgres and Redis
    async fn save_order<E: EntityForSave>(
        &self,
        order: &E,
        order_uid: &str,
        value: &str,
    ) -> Result<(), RemoteRepositoryError>;
    //method for get order on order_uid from Redis/Postrges
    async fn get_order<T>(
        &self,
        order_uid: &str,
    ) -> Result<RemoteRepositoryResponse<T>, RemoteRepositoryError>
    where
        T: for<'row> FromRow<'row, PgRow> + Send + Unpin;
    //method for get order from redis (use for background tasks)
    async fn save_order_in_redis(
        &self,
        order_uid: &str,
        order: &str,
    ) -> Result<(), RemoteRepositoryError>;
}
