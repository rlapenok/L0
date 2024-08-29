use serde::{Deserialize, Serialize};

use crate::contracts::models::PaymentEntity;

#[derive(Serialize, Deserialize, Debug)]
pub struct Payment {
    //todo mb create type
    transaction: String,
    request_id: Option<String>,
    //todo create type for currency
    currency: String,
    //todo create type for provider
    provider: String,
    amount: i64,
    payment_dt: i64,
    //todo cretae type for bank
    bank: String,
    delivery_cost: i64,
    goods_total: i64,
    custom_fee: i64,
}

impl PaymentEntity for Payment {
    fn get_amount(&self) -> i64 {
        self.amount
    }
    fn get_transaction(&self) -> &str {
        &self.transaction
    }
    fn get_request_id(&self) -> Option<&str> {
        self.request_id.as_deref()
    }
    fn get_currency(&self) -> &str {
        &self.currency
    }
    fn get_provider(&self) -> &str {
        &self.provider
    }
    fn get_payment_dt(&self) -> i64 {
        self.payment_dt
    }
    fn get_bank(&self) -> &str {
        &self.bank
    }
    fn get_delivery_cost(&self) -> i64 {
        self.delivery_cost
    }
    fn get_goods_total(&self) -> i64 {
        self.goods_total
    }
    fn get_custom_fee(&self) -> i64 {
        self.custom_fee
    }
}
