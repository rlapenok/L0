use std::time::Duration;
use order_presentation_local_service::LocalService;
use order_presentation_remote_service::RemoteService;
use tokio::time::interval;

use crate::{
    domain::

        services::{
            local_order_presentation_remote_services::LocalOrderRepresentationService,
            remote_order_presentation_remote_service::RemoteOrderRepresentationService,
    },
    errors::{
        remote_repository_error::RemoteRepositoryError, remote_service_error::RemoteServiceError,
    },
    model::model::Order,
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
    pub remote_service: R,
    pub local_service: L,
}

impl OrderPresentationState<RemoteSrv, LocalSrv> {
    pub fn new(remote_service: RemoteSrv, local_service: LocalSrv) -> Self {
        Self {
            remote_service,
            local_service,
        }
    }

    async fn save_in_postrges_and_redis(&self){
        let orders=self.local_service.get_postrges_from_memoty();
        if let Some(order_boxed)=orders.lock().await.pop_front(){
            if let Some(order)=order_boxed.as_any().downcast_ref::<Order>(){
                if let Err(err)=self.remote_service.save_order(order).await{
                    match err {
                        RemoteServiceError::RemoteRepositoryErrors(err) => {
                            match err {
                                RemoteRepositoryError::PostgresErrors(err) => {
                                    if let Some(err) = err.into_database_error() {
                                        println!("BackGround Task: error while save order in PostgresDB {}",err);
                                    } else {
                                        self.local_service
                                            .save_postgres_in_memory(order_boxed)
                                            .await;
                                        println!("BackGround Task: Order was save in memory")
                                    }
                                }
                                RemoteRepositoryError::RedisErrors(err) => {
                                    println!(
                                        "BackGround Task: error while save order in RedisDB {}",
                                        err
                                    );
                                    self.local_service
                                        .save_postgres_in_memory(order_boxed)
                                        .await;
                                    println!("BackGround Task: Order was save in memory")
                                }
                                RemoteRepositoryError::RedisUniqueErrorAndPosgresOk => {
                                    println!("BackGround Task: еhe order was accepted");
                                }
                            }
                        }
                        RemoteServiceError::SerderError(err) => {
                            println!("BackGround Task: error in deserialization{}", err);
                        }
                    }
                } else {
                    println!("BackGround Task: еhe order was accepted");
                }
                }
            }
            println!("asdas");
        }

    async fn save_raw_orders(&self){
        //read frm files raw orders
        self.local_service.read_raw_orders_from_file_and_save_in_memory().await;
        let mut postgres_redis_interval = interval(Duration::from_secs(1));
        let mut redis_interval = interval(Duration::from_secs(1));
        let state_postgres_redis = self.clone();
        let state_redis = self.clone();
        tokio::spawn(async move{
            loop {
                
                tokio::select! {
                  _ = postgres_redis_interval.tick()=>{
                    state_postgres_redis.save_in_postrges_and_redis().await
                  } 
                };
            }
        });
    }
    }



