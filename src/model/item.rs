use serde::{Deserialize, Serialize};
use sqlx::{
    prelude::{FromRow, Type},
    Decode,
};

use crate::domain::models::ItemEntity;

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    chrt_id: i64,
    track_number: String,
    price: i64,
    //todo mb create type
    rid: String,
    name: String,
    sale: i64,
    //todo type to know
    size: String,
    total_price: i64,
    nm_id: i64,
    //todo mb create type
    brand: String,
    status: i64,
}

impl ItemEntity for Item {
    fn get_chrt_id(&self) -> i64 {
        self.chrt_id
    }
    fn get_track_number(&self) -> &str {
        &self.track_number
    }
    fn get_price(&self) -> i64 {
        self.price
    }
    fn get_rid(&self) -> &str {
        &self.rid
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_sale(&self) -> i64 {
        self.sale
    }
    fn get_size(&self) -> &str {
        &self.size
    }
    fn get_total_price(&self) -> i64 {
        self.total_price
    }
    fn nm_id(&self) -> i64 {
        self.nm_id
    }
    fn get_brand(&self) -> &str {
        &self.brand
    }
    fn get_status(&self) -> i64 {
        self.status
    }
}
