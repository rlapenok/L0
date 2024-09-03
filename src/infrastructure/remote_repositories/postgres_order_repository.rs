use sqlx::{postgres::PgRow, types::Json, FromRow, Pool, Postgres};

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
    async fn save_order<E>(&self, data: &E) -> Result<(), sqlx::Error>
    where
        E: EntityForSave,
    {
        let mut tx = self.0.begin().await.inspect_err(|err| {
            println!("🚨 Error starting a database transaction:{}", err);
        })?;

        let items = Json(data.get_items());
        let delivery = Json(data.get_delivery());
        let payment = Json(data.get_payment());

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
        .bind(data.get_order_uid())
        .bind(data.get_track_number())
        .bind(data.get_entry())
        .bind(delivery)
        .bind(payment)
        .bind(items)
        .bind(data.get_locale())
        .bind(data.get_internal_signature())
        .bind(data.get_customer_id())
        .bind(data.get_delivery_service())
        .bind(data.get_shardkey())
        .bind(data.get_sm_id())
        .bind(data.get_date_created())
        .bind(data.get_oof_shard())
        .execute(&mut *tx)
        .await?;

        tx.commit().await.inspect_err(|err| {
            println!("🚨 Error commiting transaction in database :{}", err);
        })?;
        Ok(())
    }
    async fn get_order<T>(&self, data_uid: &str) -> Result<T, sqlx::Error>
    where
        T: for<'a> FromRow<'a, PgRow> + Send + Unpin,
    {
        let mut tx = self.0.begin().await.inspect_err(|err| {
            println!("🚨 Error starting a database transaction:{}", err);
        })?;

        let result = sqlx::query_as::<_, T>("SELECT * FROM orders WHERE order_uid=$1")
            .bind(data_uid)
            .fetch_one(&mut *tx)
            .await
            .inspect_err(|err| {
                println!("Error while execute search query in orders table {}", err);
            })?;

        tx.commit().await.inspect_err(|err| {
            println!("🚨 Error commiting transaction in database :{}", err);
        })?;
        Ok(result)
    }
}

/*let result=sqlx::query_as::<Postgres, T>(
            "
                WITH item_temp AS (
            SELECT rder_uid,
        json_agg(
            json_build_object(
                'chrt_id', chrt_id,
                'track_number', track_number,
                'price', price,
                'rid', rid,
                'name', name,
                'sale', sale,
                'size', size,
                'total_price', total_price,
                'nm_id', nm_id,
                'brand', brand,
                'status', status
            )
        ) AS items
    FROM
        items
    GROUP BY
        order_uid
),
delivery_temp AS (
            SELECT * FROM deliveries
),
payment_temp AS (
            SELECT * FROM payments
)
                SELECT o.order_uid, o.track_number, o.entry, o.locale, o.internal_signature,
                       o.customer_id, o.delivery_service, o.shardkey, o.sm_id, o.date_created,
                       o.oof_shard, COALESCE(i.items, '[]'::json) AS items,d.name, d.phone,
                       d.zip,d.city,d.address,d.region,d.email,p.transaction,p.request_id, p.currency
                       p.provider, p.amouunt, p.payment_dt, p.bank, p.delivery_cost, p.goods_total, p.custom_fee
                FROM orders o
                INNER  JOIN item_temp i ON o.order_uid = i.order_uid
                INNER  JOIN delivery_temp d ON o.order_uid = d.order_uid
                INNER  JOIN payment_temp t ON o.order_uid = p.order_uid
                WHERE o.order_uid = $1;
        ",
        ).bind(data_uid).fetch_one(&mut *tx).await.inspect_err(|err| {
            println!(
                "Error while execute search query in databsae {}",
                err
            )
        })?;*/

/*   //insert order into orders_table
    sqlx::query(
        "
         INSERT INTO orders (
    order_uid, track_number, entry, locale, internal_signature,
    customer_id, delivery_service, shardkey, sm_id, date_created, oof_shard
    ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)

     ",
    )
    .bind(order_uid)
    .bind(data.get_track_number())
    .bind(data.get_entry())
    .bind(data.get_locale())
    .bind(data.get_internal_signature())
    .bind(data.get_customer_id())
    .bind(data.get_delivery_service())
    .bind(data.get_shardkey())
    .bind(data.get_sm_id())
    .bind(data.get_date_created())
    .bind(data.get_oof_shard())
    .execute(&mut *tx)
    .await
    .inspect_err(|err| println!("Error while execute insert query in orders tables {}", err))?;
    //insert items into items table
    let items = data.get_items();
    sqlx::QueryBuilder::<Postgres>::new(
        "
      INSERT INTO items (
    order_uid, chrt_id, track_number, price, rid,
    name, sale, size, total_price, nm_id, brand, status
)
    ",
    )
    .push_values(items, |mut build, item| {
        build
            .push_bind(order_uid)
            .push_bind(item.get_chrt_id())
            .push_bind(item.get_track_number())
            .push_bind(item.get_price())
            .push_bind(item.get_rid())
            .push_bind(item.get_name())
            .push_bind(item.get_sale())
            .push_bind(item.get_size())
            .push_bind(item.get_total_price())
            .push_bind(item.nm_id())
            .push_bind(item.get_brand())
            .push_bind(item.get_status());
    })
    .build()
    .execute(&mut *tx)
    .await
    .inspect_err(|err| println!("Error while execute insert query in items tables {}", err))?;
    //insert payment in payments table
    let payment = data.get_payment();
    sqlx::query(
        "
     INSERT INTO payments (
    order_uid, transaction, request_id, currency, provider, amount,
    payment_dt, bank, delivery_cost, goods_total, custom_fee
) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
    ",
    )
    .bind(order_uid)
    .bind(payment.get_transaction())
    .bind(payment.get_request_id())
    .bind(payment.get_currency())
    .bind(payment.get_provider())
    .bind(payment.get_amount())
    .bind(payment.get_payment_dt())
    .bind(payment.get_bank())
    .bind(payment.get_delivery_cost())
    .bind(payment.get_goods_total())
    .bind(payment.get_custom_fee())
    .execute(&mut *tx)
    .await
    .inspect_err(|err| {
        println!(
            "Error while execute insert query in payments tables {}",
            err
        )
    })?;
    //insert delivery in deliveries table
    let delivery = data.get_delivery();
    sqlx::query(
        "
        INSERT INTO deliveries (
    order_uid, name, phone, zip, city, address, region, email
) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
    ",
    )
    .bind(order_uid)
    .bind(delivery.get_name())
    .bind(delivery.get_phone())
    .bind(delivery.get_zip())
    .bind(delivery.get_city())
    .bind(delivery.get_address())
    .bind(delivery.get_region())
    .bind(delivery.get_email())
    .execute(&mut *tx)
    .await
    .inspect_err(|err| {
        println!(
            "Error while execute insert query in delivery tables {}",
            err
        )
    })?;*/
