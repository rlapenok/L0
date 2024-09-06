use std::collections::VecDeque;

use tracing::{debug, instrument};

use crate::{
    domain::{
        local_repositories::{
            in_memory_order_presentation_repository::{Entity, RawOrdersInMemory},
            OrderPresentationLocalRepository,
        },
        models::Destination,
        services::local_order_presentation_remote_services::LocalOrderRepresentationService,
    },
    errors::local_service_error::LocalServiceErrors,
    models::model::Order,
};

#[derive(Clone)]
pub struct LocalService<T>
where
    T: Send + Sync + Clone,
{
    repo: T,
}

impl<T> LocalService<T>
where
    T: OrderPresentationLocalRepository + Send + Sync + Clone,
{
    pub fn new(repo: T) -> Self {
        Self { repo }
    }
}

impl<T> LocalOrderRepresentationService for LocalService<T>
where
    T: OrderPresentationLocalRepository + Send + Sync + Clone,
{
    #[instrument(
        skip(self),
        name = "LocalOrderRepresentationService::read_raw_orders_from_file_and_save_in_memory"
    )]
    async fn read_raw_orders_from_file_and_save_in_memory(&self) -> Result<(), LocalServiceErrors> {
        debug!("Starting to read raw orders from files");
        let raw_orders = self
            .repo
            .read_raw_orders_from_files()
            .await
            .inspect_err(|err| {
                debug!("Error when reading raw orders:{}", err);
            })?;
        let postgres_orders = raw_orders.0.map_or(VecDeque::new(), |orders| {
            orders
                .into_iter()
                .filter_map(|order| {
                    serde_json::from_str::<Order>(&order)
                        .ok()
                        .map(|order| Box::new(order) as Box<Entity>)
                })
                .collect()
        });
        let redis_orders = raw_orders.1.map_or(VecDeque::new(), |orders| {
            orders
                .into_iter()
                .filter_map(|order| {
                    serde_json::from_str::<Order>(&order)
                        .ok()
                        .map(|order| Box::new(order) as Box<Entity>)
                })
                .collect()
        });
        debug!("Orders from file will be converted to trait Object");
        debug!("Starting saving raw orders in memory");
        self.repo
            .save_orders_in_memory((postgres_orders, redis_orders))
            .await;
        debug!("Orders were successfully read from files and stored in memory");
        Ok(())
    }

    fn get_raw_orders_from_memory(&self, dest: Destination) -> RawOrdersInMemory {
        self.repo.get_raw_orders_from_memory(dest)
    }
    async fn save_raw_orders_in_file(&self, orders: &mut VecDeque<Box<Entity>>, dest: Destination) {
        let orders = orders
            .iter_mut()
            .filter_map(|order| {
                order
                    .as_any()
                    .downcast_ref::<Order>()
                    .and_then(|order| serde_json::to_string_pretty(order).ok())
            })
            .collect::<VecDeque<String>>();
        self.repo.save_row_orders_in_file(orders, dest).await
    }
    async fn save_in_memory(&self, dest: Destination, order: Box<Entity>) {
        self.repo.save_in_memory(dest, order).await
    }
}
