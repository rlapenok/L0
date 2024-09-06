use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, FromRow};
use tracing::{debug, error, instrument};

use crate::{
    domain::{
        models::EntityForSave, remote_repositories::OrderPresentationRemoteRepository,
        services::remote_order_presentation_remote_service::RemoteOrderRepresentationService,
    },
    errors::{
        remote_repository_error::RemoteRepositoryResponse, remote_service_error::RemoteServiceError,
    },
};

#[derive(Clone)]
pub struct RemoteService<R>
where
    R: OrderPresentationRemoteRepository + Send + Sync + Clone,
{
    repo: R,
}

impl<T> RemoteService<T>
where
    T: OrderPresentationRemoteRepository + Send + Sync + Clone,
{
    pub fn new(repo: T) -> Self {
        Self { repo }
    }
}

impl<R> RemoteOrderRepresentationService for RemoteService<R>
where
    R: OrderPresentationRemoteRepository + Send + Sync + Clone,
{
    #[instrument(
        skip(self, order),
        name = "RemoteOrderRepresentationService::save_order"
    )]
    async fn save_order<E: EntityForSave + Serialize>(
        &self,
        order: &E,
    ) -> Result<(), RemoteServiceError> {
        debug!("Start save order");
        //serialize order
        let order_uid = order.get_order_uid();
        let serialize_data = serde_json::to_string_pretty(order)
            .inspect_err(|err| error!("Serialization error:{}", err))?;
        debug!("Serialization was successful");
        //save order in remote repository
        self.repo
            .save_order(order, order_uid, &serialize_data)
            .await
            .inspect_err(|err| debug!("Error when saving order:{}", err))?;
        debug!("Order was saved");
        Ok(())
    }
    #[instrument(skip(self), name = "RemoteOrderRepresentationService::get_order")]
    async fn get_order<T>(&self, order_uid: &str) -> Result<(T, Option<String>), RemoteServiceError>
    where
        T: for<'de> Deserialize<'de> + for<'row> FromRow<'row, PgRow> + Send + Unpin,
    {
        debug!("Start get order");
        //get order from remote repository
        let result = self
            .repo
            .get_order::<T>(order_uid)
            .await
            .inspect_err(|err| debug!("Error when get order:{}", err))?;
        //matching result
        match result {
            RemoteRepositoryResponse::OrderFromRedis(order) => {
                debug!("Order was get from Redis");
                let order = serde_json::from_str::<T>(&order)
                    .inspect_err(|err| debug!("Deserialization error:{}", err))?;
                debug!("Order was get and deserialize from Redis");
                Ok((order, None))
            }
            RemoteRepositoryResponse::OrderFromPostgres(order, context) => {
                debug!("Order was get from Postgres with Redis Error:{}", context);

                Ok((order, Some(context)))
            }
        }
    }
    async fn save_order_in_redis<E: EntityForSave + Serialize>(
        &self,
        order: &E,
    ) -> Result<(), RemoteServiceError> {
        debug!("Start save order");
        let order_uid = order.get_order_uid();
        //serialize order
        let order = serde_json::to_string_pretty(order)
            .inspect_err(|err| debug!("Serialization error:{}", err))?;
        //save order in redis
        self.repo
            .save_order_in_redis(order_uid, &order)
            .await
            .inspect_err(|err| debug!("Error when save order fron Redis:{}", err))?;
        debug!("Order was save");
        Ok(())
    }
}
