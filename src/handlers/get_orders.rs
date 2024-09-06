use axum::extract::{Query, State};
use tracing::{error, info, instrument};

use crate::{
    errors::remote_service_error::RemoteServiceErrorResponse,
    infrastructure::services::{LocalSrv, OrderPresentationState, RemoteSrv},
    models::{
        model::Order,
        query_params::QueryParams,
        responses::{OrderResponse, OrderStatus},
    },
};
//handler for get order on order_uid
#[instrument(skip(state,params),fields(order_uid=params.get_order_uid()),name="get_order")]
pub async fn get_orders(
    State(state): State<OrderPresentationState<RemoteSrv, LocalSrv>>,
    Query(params): Query<QueryParams>,
) -> Result<OrderResponse, RemoteServiceErrorResponse> {
    let order_uid = params.get_order_uid();
    info!("Start get order");
    let result = state
        .get_order::<Order>(order_uid)
        .await
        .inspect_err(|err| error!("Error while get_order:{}", err))?;
    info!("Get order-OK");
    Ok(OrderResponse::new(
        Some(result.0),
        None,
        result.1,
        None,
        OrderStatus::Accepted,
    ))
}
