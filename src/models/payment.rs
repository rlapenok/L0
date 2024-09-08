use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::domain::models::PaymentEntity;

#[derive(Serialize, Deserialize, Validate,Debug)]
#[serde(deny_unknown_fields)]
pub struct Payment {
    #[validate(length(min = 1, message = "Can not be empty"))]
    transaction: String,
    request_id: Option<String>,
    #[validate(length(min = 1, message = "Can not be empty"))]
    currency: String,
    #[validate(length(min = 1, message = "Can not be empty"))]
    provider: String,
    amount: i64,
    payment_dt: i64,
    #[validate(length(min = 1, message = "Can not be empty"))]
    bank: String,
    delivery_cost: i64,
    goods_total: i64,
    custom_fee: i64,
}

impl Payment {
    pub fn new(
        transaction: String,
        request_id: Option<String>,
        currency: String,
        provider: String,
        amount: i64,
        payment_dt: i64,
        bank: String,
        delivery_cost: i64,
        goods_total: i64,
        custom_fee: i64,
    ) -> Self {
        Payment {
            transaction,
            request_id,
            currency,
            provider,
            amount,
            payment_dt,
            bank,
            delivery_cost,
            goods_total,
            custom_fee,
        }
    }
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
