use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    domain::services::OrderPresentationState,
    handlers::{get_order::get_order, save_order::save_order},
};

pub fn create_app<T>() -> Router<T>
where
    T: OrderPresentationState + Send + Sync + Clone + 'static,
{
    Router::new()
        .route("/save_order", post(save_order::<T>))
        .route("/get_order", get(get_order::<T>))
}
