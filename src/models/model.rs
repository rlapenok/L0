use axum::{
    extract::{rejection::JsonRejection, FromRequest, Request},
    http::StatusCode,
    Json,
};
use chrono::{DateTime, Utc};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use sqlx::{postgres::PgRow, prelude::FromRow, types::Json as SqlxJson, Row};
use validator::Validate;

use super::{delivery::Delivery, item::Item, payment::Payment};
use crate::{domain::models::EntityForSave, utils::serde_deserde_date_time};

#[derive(Serialize, Deserialize, Debug, Validate)]
#[serde(deny_unknown_fields)]
pub struct Order {
    #[validate(length(min = 1, message = "Can not be empty"))]
    order_uid: String,
    #[validate(length(min = 1, message = "Can not be empty"))]
    track_number: String,
    #[validate(length(min = 1, message = "Can not be empty"))]
    entry: String,
    #[validate(nested)]
    delivery: Delivery,
    #[validate(nested)]
    payment: Payment,
    #[validate(nested)]
    items: Vec<Item>,
    #[validate(length(min = 1, message = "Can not be empty"))]
    locale: String,
    internal_signature: Option<String>,
    #[validate(length(min = 1, message = "Can not be empty"))]
    customer_id: String,
    #[validate(length(min = 1, message = "Can not be empty"))]
    delivery_service: String,
    #[validate(length(min = 1, message = "Can not be empty"))]
    shardkey: String,
    sm_id: i64,
    //todo create Date
    #[serde(with = "serde_deserde_date_time")]
    date_created: DateTime<Utc>,
    #[validate(length(min = 1, message = "Can not be empty"))]
    oof_shard: String,
}

impl<'r> FromRow<'r, PgRow> for Order {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let delivery = row.try_get::<SqlxJson<Delivery>, &str>("delivery")?.0;
        let payment = row.try_get::<SqlxJson<Payment>, &str>("payment")?.0;
        let items = row.try_get::<SqlxJson<Vec<Item>>, &str>("items")?.0;
        Ok(Self {
            order_uid: row.try_get("order_uid")?,
            track_number: row.try_get("track_number")?,
            entry: row.try_get("entry")?,
            delivery,
            payment,
            items,
            locale: row.try_get("locale")?,
            internal_signature: row.try_get("internal_signature")?,
            customer_id: row.try_get("customer_id")?,
            delivery_service: row.try_get("delivery_service")?,
            shardkey: row.try_get("shardkey")?,
            sm_id: row.try_get("sm_id")?,
            date_created: row.try_get("date_created")?,
            oof_shard: row.try_get("oof_shard")?,
        })
    }
}

pub struct JsonOrder<J>(pub J);

#[async_trait::async_trait]
impl<T, S> FromRequest<S> for JsonOrder<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Validate,
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(data) = Json::<T>::from_request(req, state)
            .await
            .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid JSON body"))?;
        data.validate()
            .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid JSON body"))?;
        Ok(Self(data))
    }
}

impl EntityForSave for Order {
    type Delivery = Delivery;
    type Item = Item;
    type Payment = Payment;
    fn get_order_uid(&self) -> &str {
        &self.order_uid
    }
    fn get_track_number(&self) -> &str {
        &self.track_number
    }
    fn get_entry(&self) -> &str {
        &self.entry
    }
    fn get_delivery(&self) -> &Self::Delivery {
        &self.delivery
    }
    fn get_payment(&self) -> &Self::Payment {
        &self.payment
    }
    fn get_items(&self) -> &[Self::Item] {
        &self.items
    }
    fn get_locale(&self) -> &str {
        &self.locale
    }
    fn get_internal_signature(&self) -> Option<&str> {
        self.internal_signature.as_deref()
    }
    fn get_customer_id(&self) -> &str {
        &self.customer_id
    }
    fn get_delivery_service(&self) -> &str {
        &self.delivery_service
    }
    fn get_shardkey(&self) -> &str {
        &self.shardkey
    }
    fn get_sm_id(&self) -> i64 {
        self.sm_id
    }
    fn get_date_created(&self) -> DateTime<Utc> {
        self.date_created
    }
    fn get_oof_shard(&self) -> &str {
        &self.oof_shard
    }
}
