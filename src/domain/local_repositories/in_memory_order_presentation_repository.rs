use std::{collections::VecDeque, sync::Arc};

use tokio::sync::Mutex;

use crate::{domain::models::EntityForSave, model::{delivery::Delivery, item::Item, payment::Payment}};

pub type Entity=dyn EntityForSave<Payment = Payment,Delivery = Delivery,Item = Item>+Sync+Send;



pub type PostgresRawDataInMemory = Arc<Mutex<VecDeque<Box<Entity>>>>;
pub type RedisRawDataInMemory = Arc<Mutex<VecDeque<Box<Entity>>>>;

pub trait InMemoryOrderPresentationRepository: Send + Sync + Clone {
    async fn save_raw_orders(&self, orders: (VecDeque<Box<Entity>>,VecDeque<Box<Entity>>));
    async fn get_raw_orders(self) -> (VecDeque<Box<Entity>>, VecDeque<Box<Entity>>);
    fn get_postgres_from_memory(&self) -> PostgresRawDataInMemory;
    async fn save_postgres(&self, order: Box<Entity>);
    fn get_redis_from_memory(&self) -> RedisRawDataInMemory;
    async fn save_redis(&self, order: Box<Entity>);
}
