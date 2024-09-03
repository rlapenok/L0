use axum::extract::{Query, State};

use crate::{
    domain::services::remote_order_presentation_remote_service::RemoteOrderRepresentationService,
    errors::remote_service_error::RemoteServiceError,
    infrastructure::services::{LocalSrv, OrderPresentationState, RemoteSrv},
    model::{model::Order, query_params::QueryParams},
};

pub async fn get_orders(
    State(state): State<OrderPresentationState<RemoteSrv, LocalSrv>>,
    Query(params): Query<QueryParams>,
) -> Result<Order, RemoteServiceError> {
    let order_uid = params.get_order_uid();
    let order = state
        .remote_service
        .get_order::<Order>(order_uid.to_owned())
        .await?;

    Ok(order)
}
