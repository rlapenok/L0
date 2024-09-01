
use serde::Serialize;
use sqlx::{postgres::PgRow, FromRow};

use crate::{domain::models::EntityForSave, errors::remote_repository_error::RemoteRepositoryError};

/*pub trait ToOrderRepresentationRemoteRepositoryService<S>
where
    S: RemoteOrderRepresentationService,
{
    fn to_service(self) -> Result<S, Box<dyn Error>>;
}*/

pub trait RemoteOrderRepresentationService: Send + Clone {
    async fn save_order<E: EntityForSave + Serialize>(
        &self,
        data: &E,
    ) -> Result<(), RemoteRepositoryError>;
    async fn get_order<T>(&self, data_uid: String)
    where
    T: for<'a> FromRow<'a, PgRow>+Send+Unpin;
}
