use std::error::Error;

use crate::{contracts::{models::EntityForSave, repositories::RepositoryOrderService, saver::WriterReader}, data_saver::data_saver::DataWriterReader, model::model::Order, repo::repository::RepositoryType};




#[derive(Clone)]
pub struct AppState<D,R>
where
    D: WriterReader,
    R:RepositoryOrderService
{
    pub saver: D,
    pub repo:R
}


pub type AppStateType=AppState<DataWriterReader,RepositoryType>;

impl<D,R> AppState<D,R>
where
    D: WriterReader,
    R:RepositoryOrderService

{
    pub fn new(saver: D,repo:R) -> Self {
        Self { saver,repo }
    }
    pub async fn read_row_data(&self) -> Result<Option<Vec<Order>>, Box<dyn Error>> {
        let row_data = self.saver.read().await?;
        if let Some(data) = row_data {
            let mut output = Vec::new();
            data.iter()
                .for_each(|data| match serde_json::from_str::<Order>(data) {
                    Ok(data) => output.push(data),
                    Err(err) => {
                        eprintln!(
                            "Error while deserialize data from file:{}",
                            err
                        )
                    }
                });
                println!("Data previously unprocessed by the server were processed successfully");
            return Ok(Some(output));
        }
        println!("Data that could have been the server was not found during operation");
        Ok(None)
    }
    pub async fn insert<T:EntityForSave>(&self,entity:&T)->Result<(),Box<dyn Error>>{

        let postgres_result=self.repo.add_order(entity).await?;
        Ok(postgres_result)
    }
}
