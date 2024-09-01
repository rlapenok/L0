use order_presentation_remote_service::RemoteService;

use crate::domain::services::remote_order_presentation_remote_service::RemoteOrderRepresentationService;

use super::remote_repositories::{
    postgres_order_repository::PostgresOrderRepository,
    redis_order_repository::RedisOrderRepository, RemoteRepository,
};

pub mod order_presentation_local_service;
pub mod order_presentation_remote_service;

pub type RemoteRepo = RemoteRepository<PostgresOrderRepository, RedisOrderRepository>;
pub type RemoteSrv = RemoteService<RemoteRepo>;

#[derive(Clone)]
pub struct OrderPresentationState<R, >
where
    R: RemoteOrderRepresentationService+Send + Sync+Clone,
{
    pub remote_service: R,
    //pub local_service: L,
}

impl OrderPresentationState<RemoteSrv> {
    pub fn new(order_presentation_service: RemoteSrv) -> Self {
        Self {
            remote_service: order_presentation_service,
        }
    }
}
