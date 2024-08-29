use std::{error::Error, future::Future};

use super::models::EntityForSave;

pub trait ToRepositories {
    type Output:RepositoryOrderService;
    fn to_repositories(
        &self,
    ) -> impl Future<Output = Result<Self::Output, Box<dyn Error>>>;
}

pub trait PostgresRepoOrderService {
    fn add_order<T: EntityForSave>(
        &self,
        entity: &T,
    ) -> impl Future<Output = Result<(), Box<dyn Error>>>;
}

pub trait RedisRepoOrderService {}

pub trait RepositoryOrderService {
    fn add_order<T: EntityForSave>(&self, entity: &T,)->impl Future<Output = Result<(), Box<dyn Error>>>;
}

