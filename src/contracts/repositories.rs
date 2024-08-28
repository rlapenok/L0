use std::{error::Error, future::Future};

use crate::repo::postgres::PostgresRepo;

//trait for convert config to repositories
pub trait ToRepositories {
    //method for convert struct with config to PostgresRepo
    fn to_postgres_repo(&self) -> impl Future<Output = Result<PostgresRepo, Box<dyn Error>>>;
}
