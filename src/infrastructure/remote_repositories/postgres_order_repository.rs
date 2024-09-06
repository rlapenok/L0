use sqlx::{postgres::PgRow, types::Json, FromRow, Pool, Postgres};
use tracing::{instrument, trace};

use crate::domain::{
    models::EntityForSave,
    remote_repositories::postgres_order_presentation_repository::PostgresOrderPresentationRepository,
};

#[derive(Clone)]
pub struct PostgresOrderRepository(Pool<Postgres>);

impl PostgresOrderRepository {
    pub(crate) fn new(pool: Pool<Postgres>) -> Self {
        Self(pool)
    }
}

impl PostgresOrderPresentationRepository for PostgresOrderRepository {
    #[instrument(
        skip(self, order),
        name = "PostgresOrderPresentationRepository::save_order"
    )]
    async fn save_order<E>(&self, order: &E) -> Result<(), sqlx::Error>
    where
        E: EntityForSave,
    {
        trace!("Start save order");
        let mut tx = self.0.begin().await.inspect_err(|err| {
            trace!("🚨Error starting transaction:{}", err);
        })?;

        let items = Json(order.get_items());
        let delivery = Json(order.get_delivery());
        let payment = Json(order.get_payment());

        sqlx::query(
            "
                INSERT INTO orders (
            order_uid, track_number, entry, delivery, payment, items, locale,
            internal_signature, customer_id, delivery_service, shardkey, sm_id,
            date_created, oof_shard
        ) VALUES (
            $1, $2, $3, $4, $5, $6, $7,
            $8, $9, $10, $11, $12,
            $13, $14
        )
        ",
        )
        .bind(order.get_order_uid())
        .bind(order.get_track_number())
        .bind(order.get_entry())
        .bind(delivery)
        .bind(payment)
        .bind(items)
        .bind(order.get_locale())
        .bind(order.get_internal_signature())
        .bind(order.get_customer_id())
        .bind(order.get_delivery_service())
        .bind(order.get_shardkey())
        .bind(order.get_sm_id())
        .bind(order.get_date_created())
        .bind(order.get_oof_shard())
        .execute(&mut *tx)
        .await
        .inspect_err(|err| {
            trace!("🚨Error during transaction:{}", err);
        })?;

        tx.commit().await.inspect_err(|err| {
            trace!("🚨Error commiting transaction:{}", err);
        })?;
        trace!("Order was saved");
        Ok(())
    }
    #[instrument(
        skip(self, order_uid),
        name = "PostgresOrderPresentationRepository::get_order"
    )]
    async fn get_order<T>(&self, order_uid: &str) -> Result<T, sqlx::Error>
    where
        T: for<'a> FromRow<'a, PgRow> + Send + Unpin,
    {
        trace!("Start get order");
        let mut tx = self.0.begin().await.inspect_err(|err| {
            trace!("🚨Error starting transaction:{}", err);
        })?;

        let result = sqlx::query_as::<_, T>("SELECT * FROM orders WHERE order_uid=$1")
            .bind(order_uid)
            .fetch_one(&mut *tx)
            .await
            .inspect_err(|err| {
                trace!("🚨Error during transaction:{}", err);
            })?;

        tx.commit().await.inspect_err(|err| {
            trace!("🚨Error commiting transaction:{}", err);
        })?;
        trace!("Order was get");
        Ok(result)
    }
}
