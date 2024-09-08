use mockall::automock;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, FromRow};

use crate::{errors::remote_service_error::RemoteServiceErrorResponse, models::model::Order};

use super::models::EntityForSave;

pub mod local_order_presentation_remote_services;
pub mod remote_order_presentation_remote_service;

//trait for State
#[automock]
#[async_trait::async_trait]
pub trait OrderPresentationState: Clone + Send + Sync {
    async fn save_order(&self, order: Order) -> Result<(), RemoteServiceErrorResponse>;
    async fn get_order<T>(
        &self,
        order_uid: &str,
    ) -> Result<(T, Option<String>), RemoteServiceErrorResponse>
    where
        T: for<'de> Deserialize<'de>
            + for<'row> FromRow<'row, PgRow>
            + Send
            + Sync
            + Unpin
            + EntityForSave
            + Serialize;
    async fn save_raw_orders(&self);
}

impl Clone for MockOrderPresentationState {
    fn clone(&self) -> Self {
        Self::new()
    }
}
