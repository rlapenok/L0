use axum::{extract::State, Json};

use crate::{
    domain::services::remote_order_presentation_remote_service::RemoteOrderRepresentationService,
    errors::remote_service_error::RemoteServiceError,
    infrastructure::services::{LocalSrv, OrderPresentationState, RemoteSrv},
    model::{model::Order, responses::AddOrderResponse},
};

pub async fn add_order(
    State(state): State<OrderPresentationState<RemoteSrv, LocalSrv>>,
    Json(order): Json<Order>,
) -> Result<AddOrderResponse, RemoteServiceError> {
    state.remote_service.save_order(&order).await?;
    let resposne = AddOrderResponse::default();
    Ok(resposne)
}
