use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Payment {
    transaction: String,
    request_id: Option<String>,
    //todo create type for currency
    currency: String,
    //todo create type for provider
    provider: String,
    amount: u64,
    payment_dt: u64,
    //todo cretae type for bank
    bank: String,
    delivery_cost: u64,
    goods_total: u64,
    custom_fee: u64,
}
