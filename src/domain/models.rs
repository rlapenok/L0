use chrono::{DateTime, Utc};
use downcast_rs::{impl_downcast, Downcast, DowncastSync};
use serde::Serialize;

use crate::model::{delivery::Delivery, item::Item, payment::Payment};

pub trait EntityForSave:Downcast+DowncastSync {
    type Delivery: DeliveriEntity + Serialize;
    type Payment: PaymentEntity + Serialize;
    type Item: ItemEntity + Serialize;
    fn get_order_uid(&self) -> &str;
    fn get_track_number(&self) -> &str;
    fn get_entry(&self) -> &str;
    fn get_delivery(&self) -> &Self::Delivery;
    fn get_payment(&self) -> &Self::Payment;
    fn get_items(&self) -> &[Self::Item];
    fn get_locale(&self) -> &str;
    fn get_internal_signature(&self) -> Option<&str>;
    fn get_customer_id(&self) -> &str;
    fn get_delivery_service(&self) -> &str;
    fn get_shardkey(&self) -> &str;
    fn get_sm_id(&self) -> i64;
    fn get_date_created(&self) -> DateTime<Utc>;
    fn get_oof_shard(&self) -> &str;
}


//impl_downcast!(concrete  EntityForSave assoc Payment=Payment,Delivery=Delivery,Item=Item);
impl_downcast!(sync concrete  EntityForSave assoc Payment=Payment,Delivery=Delivery,Item=Item);


pub trait ItemEntity {
    fn get_chrt_id(&self) -> i64;
    fn get_track_number(&self) -> &str;
    fn get_price(&self) -> i64;
    fn get_rid(&self) -> &str;
    fn get_name(&self) -> &str;
    fn get_sale(&self) -> i64;
    fn get_size(&self) -> &str;
    fn get_total_price(&self) -> i64;
    fn nm_id(&self) -> i64;
    fn get_brand(&self) -> &str;
    fn get_status(&self) -> i64;
}
pub trait DeliveriEntity {
    fn get_name(&self) -> &str;
    fn get_phone(&self) -> &str;
    fn get_zip(&self) -> &str;
    fn get_city(&self) -> &str;
    fn get_address(&self) -> &str;
    fn get_region(&self) -> &str;
    fn get_email(&self) -> &str;
}
pub trait PaymentEntity {
    fn get_transaction(&self) -> &str;
    fn get_request_id(&self) -> Option<&str>;
    fn get_currency(&self) -> &str;
    fn get_provider(&self) -> &str;
    fn get_amount(&self) -> i64;
    fn get_payment_dt(&self) -> i64;
    fn get_bank(&self) -> &str;
    fn get_delivery_cost(&self) -> i64;
    fn get_goods_total(&self) -> i64;
    fn get_custom_fee(&self) -> i64;
}
