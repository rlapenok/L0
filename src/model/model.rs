use serde::{Deserialize, Serialize};

use super::{delivery::Delivery, item::Item, payment::Payment};

#[derive(Serialize, Deserialize)]
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
    sm_id: u64,
    date_created: String,
    oof_shard: String,
}
