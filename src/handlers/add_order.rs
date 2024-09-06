use axum::extract::State;
use tracing::{error, info, instrument};

use crate::{
    domain::models::EntityForSave,
    errors::remote_service_error::RemoteServiceErrorResponse,
    infrastructure::services::{LocalSrv, OrderPresentationState, RemoteSrv},
    models::{
        model::{JsonOrder, Order},
        responses::OrderResponse,
    },
};
//handler for add order in databases
#[instrument(skip(state,order),fields(order_uid=order.get_order_uid()),name="add_order")]
pub async fn add_order(
    State(state): State<OrderPresentationState<RemoteSrv, LocalSrv>>,
    JsonOrder(order): JsonOrder<Order>,
) -> Result<OrderResponse, RemoteServiceErrorResponse> {
    info!("Start save order");
    state.save_order(order).await.inspect_err(|err| {
        error!("Error while save order:{}", err);
    })?;
    info!("Order save");
    Ok(OrderResponse::default())
}
