use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Serialize)]
pub enum OrderStatus {
    Accepted,
    NotAccepted,
    WillBeAccepted,
}
impl Default for OrderStatus {
    fn default() -> Self {
        Self::Accepted
    }
}

#[derive(Serialize, Default)]
pub struct AddOrderResponse {
    postgres_err: Option<String>,
    redis_err: Option<String>,
    serde_err: Option<String>,
    order_status: OrderStatus,
}
impl AddOrderResponse {
    pub fn new(
        postgres_err: Option<String>,
        redis_err: Option<String>,
        serde_err: Option<String>,
        order_status: OrderStatus,
    ) -> Self {
        Self {
            postgres_err,
            redis_err,
            serde_err,
            order_status,
        }
    }
}

impl IntoResponse for AddOrderResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}
