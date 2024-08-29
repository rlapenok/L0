use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{delivery::Delivery, item::Item, payment::Payment};
use crate::{contracts::models::EntityForSave, utils::serde_deserde_date_time};

#[derive(Serialize, Deserialize, Debug)]
pub struct Order {
    //todo create type
    order_uid: String,
    //todo create type
    track_number: String,
    entry: String,
    delivery: Delivery,
    payment: Payment,
    items: Vec<Item>,
    //todo create type
    locale: String,
    internal_signature: Option<String>,
    customer_id: String,
    //todo create type
    delivery_service: String,
    shardkey: String,
    sm_id: i64,
    //todo create Date
    #[serde(with = "serde_deserde_date_time")]
    date_created: DateTime<Utc>,
    oof_shard: String,
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
