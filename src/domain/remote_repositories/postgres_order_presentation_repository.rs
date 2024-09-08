use sqlx::{postgres::PgRow, FromRow};

use crate::domain::models::EntityForSave;

//trait for save/get order in/from Postgres
#[allow(async_fn_in_trait)]
pub trait PostgresOrderPresentationRepository: Send + Sync + Clone {
    //method for save order
    async fn save_order<E: EntityForSave>(&self, order: &E) -> Result<(), sqlx::Error>;
    //method for get order on order_uid
    async fn get_order<T>(&self, order_uid: &str) -> Result<T, sqlx::Error>
    where
        T: for<'a> FromRow<'a, PgRow> + Send + Unpin;
}
