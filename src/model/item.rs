use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Item {
    chrt_id: u64,
    track_number: String,
    price: i64,
    rid: String,
    name: String,
    sale: u64,
    size: String,
    total_price: u64,
    nm_id: u64,
    brand: String,
    status: u64,
}
