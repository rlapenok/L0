use crate::contracts::repositories::RedisRepoOrderService;



#[derive(Clone)]
pub struct RedisRepo();


impl RedisRepo {
    pub fn new(pool: ()) -> Self {
        Self()
    }
}

impl RedisRepoOrderService for RedisRepo {}
