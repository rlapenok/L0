use std::{collections::VecDeque, error::Error, future::Future};

use serde::Serialize;
use sqlx::{postgres::PgRow, FromRow};

use crate::{
    domain::{
        models::EntityForSave,
        remote_repositories::{Destination, OrderPresentationRemoteRepository},
        services::remote_order_presentation_remote_service::RemoteOrderRepresentationService,
    },
    errors::remote_repository_error::RemoteRepositoryError,
    model::model::Order,
};

#[derive(Clone)]
pub(crate) struct RemoteService<R>
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
impl<R> RemoteService<R>
where
    R: OrderPresentationRemoteRepository + Send + Sync + Clone,
{
    async fn save_row_data<E>(&self, data: &mut VecDeque<E>)
    where
        E: EntityForSave + Serialize,
    {
        while let Some(entity) = data.pop_front() {
            let result = self.save_order(&entity).await;
        }
    }
}

impl<R> RemoteOrderRepresentationService for RemoteService<R>
where
    R: OrderPresentationRemoteRepository + Send + Sync + Clone,
{
    async fn save_order<E: EntityForSave + Serialize>(
        &self,
        entity: &E,
    ) -> Result<(), RemoteRepositoryError> {
        let order_uid = entity.get_order_uid();
        let serialize_data = serde_json::to_string_pretty(entity).unwrap();
        if let Err(err) = self
            .repo
            .save_order(entity, order_uid, &serialize_data)
            .await
        {
            match err {
                RemoteRepositoryError::PostgresRepositoryErrors(err) => {
                    match err {
                        sqlx::Error::Database(err) => {
                            if !err.is_unique_violation() {
                                //todo return error InternalError order will be save later
                                //self.remote_repo.save_order_local(serialize_data, Files::PostgresFile).await?;
                                return todo!();
                            }
                            //todo return error InternalError  order_uid already exist
                            return todo!();
                        }
                        _ => {
                            //todo return error InternalError order will be save later
                            //self.remote_repo.save_order_local(serialize_data, Files::PostgresFile).await?;
                            return todo!();
                        }
                    }
                }
                RemoteRepositoryError::RedisRepositoryErrors(err) => {
                    //todo return internal error message save in postgres but not save in redis
                    //self.remote_repo.save_order_local(serialize_data, Files::RedisFile).await?;
                    println!("{}", err);
                    todo!()
                }
            }
        }
        Ok(())
    }
    async fn get_order<T>(&self, data_uid: String)
        where
        T: for<'a> FromRow<'a, PgRow>+Send+Unpin {

        let b=self.repo.get_order::<T>(data_uid).await;    
        
    }

    /*fn read_and_save_row_data(&self) -> impl Future<Output = Result<(), Box<dyn Error>>> + Send {
        async {
            println!("Start read and save row data");
            let row_data = self.remote_repo.read_row_data().await?;
            if let Some(data) = row_data {
                let mut output = VecDeque::new();
                data.iter()
                    .for_each(|data| match serde_json::from_str::<Order>(data) {
                        Ok(data) => output.push_back(data),
                        Err(err) => {
                            eprintln!("Error while deserialize data from file:{}", err)
                        }
                    });
                println!("Data previously unprocessed by the server were processed successfully");
                todo!()
            }
            println!("Data that could have been the server was not found during operation");
            return Ok(());
        }
    }*/
}
