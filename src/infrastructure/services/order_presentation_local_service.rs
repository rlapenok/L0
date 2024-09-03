use std::collections::VecDeque;

use crate::{
    domain::{
        local_repositories::{
            in_memory_order_presentation_repository::{
                Entity, PostgresRawDataInMemory, RedisRawDataInMemory
            },
            OrderPresentationLocalRepository,
        }, services::local_order_presentation_remote_services::LocalOrderRepresentationService
    },
    errors::local_service_error::LocalServiceErrors, model::model::Order,
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
    async fn read_raw_orders_from_file_and_save_in_memory(&self) -> Result<(), LocalServiceErrors> 
    {
       let raw_data=self.repo.read_raw_orders_from_files().await?;
       let postgres_orders=raw_data.0.map_or(VecDeque::new(), |orders|{
            orders.into_iter().filter_map(|order|{
                serde_json::from_str::<Order>(&order).ok().map(|order|{
                    Box::new(order) as Box<Entity>
                })
            }).collect()
            
       });
       let redis_orders=raw_data.1.map_or(VecDeque::new(), |orders|{
        orders.into_iter().filter_map(|order|{
            serde_json::from_str::<Order>(&order).ok().map(|order|{
                Box::new(order) as Box<Entity>
            })
        }).collect()});
        self.repo.save_orders_in_memory((postgres_orders,redis_orders)).await;
        Ok(())
    }

    fn get_postrges_from_memoty(&self) -> PostgresRawDataInMemory {
        self.repo.get_postgres_from_memory()
    }

    async fn save_postgres_in_memory(&self, order: Box<Entity>) {
        self.repo.save_postgres_in_memory(order).await;
    }
   fn get_redis_from_memory(&self) -> RedisRawDataInMemory {
       self.repo.get_redis_from_memory()
   }
    async fn save_redis_in_memory(&self, order: Box<Entity>) {
        self.repo.save_redis_in_memory(order).await
    }
}
