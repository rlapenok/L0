use axum::extract::State;
use tracing::{error, info, instrument};

use crate::{
    domain::{models::EntityForSave, services::OrderPresentationState},
    errors::remote_service_error::RemoteServiceErrorResponse,
    models::{
        model::{JsonOrder, Order},
        responses::OrderResponse,
    },
};

//handler for add order in databases
#[instrument(skip(state,order),fields(order_uid=order.get_order_uid()),name="add_order")]
pub async fn save_order<T>(
    State(state): State<T>,
    JsonOrder(order): JsonOrder<Order>,
) -> Result<OrderResponse, RemoteServiceErrorResponse>
where
    T: OrderPresentationState + Send + Sync + 'static,
{
    info!("Start save order");
    state.save_order(order).await.inspect_err(|err| {
        error!("Error while save order:{}", err);
    })?;
    info!("Order save");
    Ok(OrderResponse::default())
}
