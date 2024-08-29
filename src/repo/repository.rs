use std::{error::Error, future::Future};

use sqlx::{Pool, Postgres};

use crate::contracts::{models::EntityForSave, repositories::{
    PostgresRepoOrderService, RedisRepoOrderService, RepositoryOrderService,
}};

use super::{postgres::PostgresRepo, redis::RedisRepo};

pub type RepositoryType=Repository<PostgresRepo,RedisRepo>;


#[derive(Clone)]
pub struct Repository<P, R>
where
    P: PostgresRepoOrderService,
    R: RedisRepoOrderService,
{
    postgres: P,
    redis: R,
}

impl Repository<PostgresRepo, RedisRepo> {
    pub fn new(pool: Pool<Postgres>, redis: ()) -> Self {
        let postgres = PostgresRepo::new(pool);
        let redis = RedisRepo::new(redis);
        Self { postgres, redis }
    }
}

impl RepositoryOrderService for Repository<PostgresRepo, RedisRepo> {

    fn add_order<T: EntityForSave>(&self, entity: &T,)->impl Future<Output = Result<(), Box<dyn Error>>> {
        async {
            self.postgres.add_order(entity).await
        }
    }
}
