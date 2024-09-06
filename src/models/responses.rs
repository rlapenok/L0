use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

use super::model::Order;

#[derive(Serialize)]
pub enum OrderStatus {
    Accepted,
    NotAccepted,
    WillBeAccepted,
    Unknown,
}
impl Default for OrderStatus {
    fn default() -> Self {
        Self::Accepted
    }
}

#[derive(Serialize, Default)]
pub struct OrderResponse {
    order: Option<Order>,
    postgres_err: Option<String>,
    redis_err: Option<String>,
    serde_err: Option<String>,
    order_status: OrderStatus,
}
impl OrderResponse {
    pub fn new(
        order: Option<Order>,
        postgres_err: Option<String>,
        redis_err: Option<String>,
        serde_err: Option<String>,
        order_status: OrderStatus,
    ) -> Self {
        Self {
            order,
            postgres_err,
            redis_err,
            serde_err,
            order_status,
        }
    }
    fn is_redis_err(&self) -> bool {
        self.redis_err.is_some()
    }
}

impl IntoResponse for OrderResponse {
    fn into_response(self) -> Response {
        if self.is_redis_err() {
            (StatusCode::MULTI_STATUS, Json(self)).into_response()
        } else {
            (StatusCode::OK, Json(self)).into_response()
        }
    }
}
