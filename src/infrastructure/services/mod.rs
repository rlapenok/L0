use order_presentation_local_service::LocalService;
use order_presentation_remote_service::RemoteService;
use serde::Deserialize;
use sqlx::{postgres::PgRow, FromRow};
use std::{ops::Deref, sync::Arc, time::Duration};
use tokio::{signal, sync::Mutex, task::JoinSet, time::interval};
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info, info_span, Instrument};

use crate::{
    domain::{
        models::Destination,
        services::{
            local_order_presentation_remote_services::LocalOrderRepresentationService,
            remote_order_presentation_remote_service::RemoteOrderRepresentationService,
        },
    },
    errors::{
        remote_repository_error::RemoteRepositoryError,
        remote_service_error::{Handler, RemoteServiceError, RemoteServiceErrorResponse},
    },
    models::model::Order,
};

use super::{
    local_repositories::LocalRepository,
    remote_repositories::{
        postgres_order_repository::PostgresOrderRepository,
        redis_order_repository::RedisOrderRepository, RemoteRepository,
    },
};

pub mod order_presentation_local_service;
pub mod order_presentation_remote_service;

pub type RemoteRepo = RemoteRepository<PostgresOrderRepository, RedisOrderRepository>;
pub type RemoteSrv = RemoteService<RemoteRepo>;
pub type LocalSrv = LocalService<LocalRepository>;

#[derive(Clone)]
pub struct OrderPresentationState<R, L>
where
    R: RemoteOrderRepresentationService + Send + Sync + Clone,
    L: LocalOrderRepresentationService + Send + Sync + Clone,
{
    remote_service: R,
    local_service: L,
    token: CancellationToken,
    postgres_cancellation_token: CancellationToken,
    redis_cancellation_token: CancellationToken,
    handle: Arc<Mutex<JoinSet<()>>>,
}

impl OrderPresentationState<RemoteSrv, LocalSrv> {
    pub fn new(remote_service: RemoteSrv, local_service: LocalSrv) -> Self {
        let token = CancellationToken::new();
        let postgres_cancellation_token = token.child_token();
        let redis_cancellation_token = token.child_token();
        Self {
            remote_service,
            local_service,
            token,
            postgres_cancellation_token,
            redis_cancellation_token,
            handle: Arc::new(Mutex::new(JoinSet::new())),
        }
    }

    pub async fn save_order(&self, order: Order) -> Result<(), RemoteServiceErrorResponse> {
        match self.remote_service.save_order(&order).await {
            Ok(_) => Ok(()),
            Err(service_err) => match &service_err {
                RemoteServiceError::RemoteRepositoryErrors(repo_err) => match repo_err {
                    RemoteRepositoryError::PostgresErrors(err) => {
                        if let Some(err) = err.as_database_error() {
                            error!("Error while save order in Postgres:{}", err);
                            Err(RemoteServiceErrorResponse::new(
                                Handler::GetOrder,
                                service_err,
                            ))
                        } else {
                            let order = Box::new(order);
                            info!("Save order in memory....");
                            self.local_service
                                .save_in_memory(Destination::Postgres, order)
                                .await;
                            info!("Order save in memory for postgres and redis");
                            Err(RemoteServiceErrorResponse::new(
                                Handler::AddOrder,
                                service_err,
                            ))
                        }
                    }
                    RemoteRepositoryError::RedisErrors(err) => {
                        error!("Error while save order in Redis:{}", err);
                        let order = Box::new(order);
                        info!("Save order in memory....");
                        self.local_service
                            .save_in_memory(Destination::Redis, order)
                            .await;
                        info!("Order save in memory for redis");
                        Err(RemoteServiceErrorResponse::new(
                            Handler::AddOrder,
                            service_err,
                        ))
                    }
                    RemoteRepositoryError::RedisUniqueErrorAndPosgresOk => Ok(()),
                },
                _ => Err(RemoteServiceErrorResponse::new(
                    Handler::AddOrder,
                    service_err,
                )),
            },
        }
    }
    pub async fn get_order<E>(
        &self,
        order_uid: &str,
    ) -> Result<(E, Option<String>), RemoteServiceErrorResponse>
    where
        E: for<'de> Deserialize<'de> + for<'row> FromRow<'row, PgRow> + Send + Unpin,
    {
        match self.remote_service.get_order::<E>(order_uid).await {
            Ok(response) => Ok(response),
            Err(err) => Err(RemoteServiceErrorResponse::new(Handler::GetOrder, err)),
        }
    }

    pub async fn graceful_shutdown(&self) {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
        self.token.cancel();
        let mut guard = self.handle.lock().await;
        while let Some(res) = guard.join_next().await {
            if let Err(err) = res {
                error!("Error while join background task:{}", err)
            }
        }
    }
    async fn save_in_redis(&self) {
        debug!("Receive row orders");
        let orders = self
            .local_service
            .get_raw_orders_from_memory(Destination::Redis);
        let mut guard = orders.lock().await;
        if let Some(order_boxed) = guard.pop_front() {
            debug!("Order found");
            if let Some(order) = order_boxed.deref().as_any().downcast_ref::<Order>() {
                if let Err(err) = self.remote_service.save_order_in_redis(order).await {
                    if err.is_unique_redis_err() {
                        error!("Error when saving order in Redis:{}", err);
                    } else {
                        drop(guard);
                        error!("Error when saving order in Redis:{}", err);
                        let mut guard = orders.lock().await;
                        guard.push_back(order_boxed);
                        debug!("Order save in memory");
                    }
                } else {
                    {
                        info!("Order save in Redis")
                    }
                }
            } else {
                error!("Can't cast to model 'Order' ")
            }
        } else {
            debug!("Order not found")
        }
    }
    async fn save_in_postrges_and_redis(&self) {
        debug!("Receive row orders");
        let orders = self
            .local_service
            .get_raw_orders_from_memory(Destination::Postgres);
        let mut guard = orders.lock().await;
        if let Some(order_boxed) = guard.pop_front() {
            debug!("Order found");
            if let Some(order) = order_boxed.deref().as_any().downcast_ref::<Order>() {
                if let Err(err) = self.remote_service.save_order(order).await {
                    if err.is_unique_postgres_err() {
                        error!("Error when saving order in Postgres And Redis:{}", err);
                    } else if err.is_unique_redis_err() {
                        error!("Error when saving order in Redis:{}", err);
                    } else {
                        drop(guard);
                        error!("Error when saving order in Postgres and Redis:{}", err);
                        let mut guard = orders.lock().await;
                        guard.push_back(order_boxed);
                        debug!("Order save in memory");
                    }
                } else {
                    info!("Order save in Postgres and Redis")
                }
            } else {
                error!("Can't cast to model 'Order' ")
            }
        } else {
            debug!("Order not found")
        }
    }

    pub async fn save_raw_orders(&self) {
        info!("Starting to read raw orders from files and save in memory");
        self.local_service
            .read_raw_orders_from_file_and_save_in_memory()
            .await
            .unwrap_or_else(|err| {
                error!("Error when reading orders from files:{}", err);
            });
        info!("Orders read from files and stored in memory");
        //create intervals for background tasks
        let mut postgres_redis_interval = interval(Duration::from_secs(10));
        let mut redis_interval = interval(Duration::from_secs(30));
        //get cloned for background tasks
        let state_postgres_redis = self.clone();
        let state_redis = self.clone();
        let backgound_task1=async move {
            loop {
                tokio::select! {
                  _ = postgres_redis_interval.tick()=>{
                    state_postgres_redis.save_in_postrges_and_redis().instrument(info_span!("PostgresAndRedisBackGroundTask")).await
                  }
                  _ = state_postgres_redis.postgres_cancellation_token.cancelled()=>{
                        info!("Start graceful shutdown");
                        info!("Starting to receive raw orders");
                        let guard=state_postgres_redis.local_service.get_raw_orders_from_memory(Destination::Postgres);
                        info!("Raw orders received");
                            let raw_orders=&mut *guard.lock().await;
                            info!("Starting to save raw orders to a file");
                            state_postgres_redis.local_service.save_raw_orders_in_file(raw_orders,Destination::Postgres).await;
                        info!("Raw orders saved to file");
                        info!("Graceful shutdown - OK");
                        break
                  }
                }
            } 
        }.instrument(info_span!("PostgresAndRedisBackGroundTask::thread"));
        let mut guard = self.handle.lock().await;
        guard.spawn(backgound_task1);
        let background_task2=async move {
            loop {            
                tokio::select! {
                    _ = redis_interval.tick()=>{
                        state_redis.save_in_redis().instrument(info_span!("RedisBackGroundTask")).await
                      }
                      _= state_redis.redis_cancellation_token.cancelled()=>{
                        info!("Start graceful shutdown");
                        info!("Starting to receive raw orders");
                        let guard=state_redis.local_service.get_raw_orders_from_memory(Destination::Redis);
                        info!("Raw orders received");
                        let raw_orders=&mut *guard.lock().await;
                        info!("Starting to save raw tokens to a file");
                        state_redis.local_service.save_raw_orders_in_file(raw_orders,Destination::Redis).await;
                        info!("Raw tokens saved to file");
                        info!("Graceful shutdown - OK");
                        break
                      }
                }
            }
        }.instrument(info_span!("RedisBackGroundTask::thread"));
        guard.spawn(background_task2);
    }
}

pub async fn graceful_shutdown_server(state: OrderPresentationState<RemoteSrv, LocalSrv>) {
    state.graceful_shutdown().await
}
