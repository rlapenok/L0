use sqlx::{postgres::PgRow, FromRow};

use crate::{
    domain::models::EntityForSave, errors::remote_repository_error::RemoteRepositoryError,
};

//trait for business logic to save/return orders in Postgres
pub trait PostgresOrderPresentationRepository: Send + Sync + Clone {
    async fn save_order<E: EntityForSave>(&self, data: &E) -> Result<(), RemoteRepositoryError>;
    async fn get_order<T>(&self, data_uid: &str) -> Result<T, RemoteRepositoryError>
    where
        T: for<'a> FromRow<'a, PgRow>+Send+Unpin;
}
