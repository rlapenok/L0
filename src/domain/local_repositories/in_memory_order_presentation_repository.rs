use std::{collections::VecDeque, sync::Arc};

use tokio::sync::Mutex;

use crate::{
    domain::models::{Destination, EntityForSave},
    models::{delivery::Delivery, item::Item, payment::Payment},
};

pub type Entity =
    dyn EntityForSave<Payment = Payment, Delivery = Delivery, Item = Item> + Sync + Send;

pub type RawOrdersInMemory = Arc<Mutex<VecDeque<Box<Entity>>>>;

//trait for save raw orders in memory after up and before the graceful shutdown and insert for background tasks(save raw orders)
pub trait InMemoryOrderPresentationRepository: Send + Sync + Clone {
    //save row orders from file in memory when server up
    async fn save_raw_orders(&self, orders: (VecDeque<Box<Entity>>, VecDeque<Box<Entity>>));
    //save row order from background task in memory
    async fn save_raw_order(&self, dest: Destination, order: Box<Entity>);
    //get row orders from memory for background task
    fn get_raw_orders(&self, dest: Destination) -> RawOrdersInMemory;
}
