use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::domain::models::ItemEntity;

#[derive(Serialize, Deserialize, Validate,Debug)]
#[serde(deny_unknown_fields)]
pub struct Item {
    chrt_id: i64,
    #[validate(length(min = 1, message = "Can not be empty"))]
    track_number: String,
    price: i64,
    #[validate(length(min = 1, message = "Can not be empty"))]
    rid: String,
    #[validate(length(min = 1, message = "Can not be empty"))]
    name: String,
    sale: i64,
    #[validate(length(min = 1, message = "Can not be empty"))]
    size: String,
    total_price: i64,
    nm_id: i64,
    #[validate(length(min = 1, message = "Can not be empty"))]
    brand: String,
    status: i64,
}
impl Item {
    pub fn new(
        chrt_id: i64,
        track_number: String,
        price: i64,
        rid: String,
        name: String,
        sale: i64,
        size: String,
        total_price: i64,
        nm_id: i64,
        brand: String,
        status: i64,
    ) -> Self {
        Item {
            chrt_id,
            track_number,
            price,
            rid,
            name,
            sale,
            size,
            total_price,
            nm_id,
            brand,
            status,
        }
    }
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
