use std::{error::Error, future::Future};

use sqlx::{Pool, Postgres};

use crate::contracts::{
    models::{DeliveriEntity, EntityForSave, ItemEntity, PaymentEntity},
    repositories::PostgresRepoOrderService,
};

#[derive(Clone)]
pub struct PostgresRepo {
    pool: Pool<Postgres>,
}

impl PostgresRepo {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

impl PostgresRepoOrderService for PostgresRepo {
    fn add_order<T: EntityForSave>(
        &self,
        entity: &T,
    ) -> impl Future<Output = Result<(), Box<dyn Error>>> {
        async {
            let mut tx = self.pool.begin().await.inspect_err(|err| {
                println!("🚨 Error starting a database transaction:{}", err);
            })?;
            let order_uid = entity.get_order_uid();
            //insert order into orders_table
            sqlx::query(
                "
                 INSERT INTO orders (
            order_uid, track_number, entry, locale, internal_signature, 
            customer_id, delivery_service, shardkey, sm_id, date_created, oof_shard
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
             
             ",
            )
            .bind(order_uid)
            .bind(entity.get_track_number())
            .bind(entity.get_entry())
            .bind(entity.get_locale())
            .bind(entity.get_internal_signature())
            .bind(entity.get_customer_id())
            .bind(entity.get_delivery_service())
            .bind(entity.get_shardkey())
            .bind(entity.get_sm_id())
            .bind(entity.get_date_created())
            .bind(entity.get_oof_shard())
            .execute(&mut *tx)
            .await
            .inspect_err(|err| {
                println!("Error while execute insert query in orders tables {}", err)
            })?;
            //insert items into items table
            let items = entity.get_items();
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
            .inspect_err(|err| {
                println!("Error while execute insert query in items tables {}", err)
            })?;
            //insert payment in payments table
            let payment = entity.get_payment();
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
            let delivery = entity.get_delivery();
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
            })?;
            tx.commit().await.inspect_err(|err| {
                println!("🚨 Error commiting transaction in database :{}", err);
            })?;
            Ok(())
        }
    }
}
