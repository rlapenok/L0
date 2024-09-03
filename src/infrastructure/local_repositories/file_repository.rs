use std::{collections::VecDeque, sync::Arc};

use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
    sync::{Mutex, MutexGuard},
};

use crate::{
    domain::{
        local_repositories::in_file_order_presentation_repository::{InFileOrderPresentationRepository, PostgresRawDataFromFile, RedisRawDataInFromFile},
        remote_repositories::Destination,
    },
    errors::local_repository_error::LocalRepositoryError,
};

#[derive(Clone)]
pub struct FileRepository {
    file_postgres: Arc<Mutex<File>>,
    file_redis: Arc<Mutex<File>>,
}
impl FileRepository {
    pub(crate) fn new(files: (File, File)) -> Self {
        Self {
            file_postgres: Arc::new(Mutex::new(files.0)),
            file_redis: Arc::new(Mutex::new(files.1)),
        }
    }
    async fn get_guard(&self, dest: Destination) -> MutexGuard<'_, File> {
        match dest {
            Destination::PostgresFile => self.file_postgres.lock().await,
            Destination::RedisFile => self.file_redis.lock().await,
        }
    }
    async fn save_row_order_in_file(
        &self,
        data: String,
        dest: Destination,
    ) -> Result<(), LocalRepositoryError> {
        let mut guard = self.get_guard(dest).await;
        let save_data = data + "\n#\n";
        guard.write_all(save_data.as_bytes()).await?;
        guard.flush().await?;
        Ok(())
    }
    async fn read_row_data_from_files(
        &self,
        dest: Destination,
    ) -> Result<Option<VecDeque<String>>, LocalRepositoryError> {
        let mut guard = self.get_guard(dest).await;

        let mut buff = String::new();
        guard.read_to_string(&mut buff).await?;
        let data = buff
            .split("\n#\n")
            .filter(|data| !data.is_empty())
            .map(|data| data.to_owned())
            .collect::<VecDeque<String>>();
        guard.set_len(0).await?;
        println!("Data previously unprocessed by the server were read successfully");
        Ok(if data.is_empty() { None } else { Some(data) })
    }
}

impl InFileOrderPresentationRepository for FileRepository {
    async fn get_raw_orders(
        &self,
    ) -> Result<(PostgresRawDataFromFile, RedisRawDataInFromFile), LocalRepositoryError> {
        let postgres_row_data = self
            .read_row_data_from_files(Destination::PostgresFile)
            .await
            .unwrap();
        let redis_row_data = self
            .read_row_data_from_files(Destination::RedisFile)
            .await
            .unwrap();
        Ok((postgres_row_data, redis_row_data))
    }
    async fn save_orders(
        &self,
        data: Vec<String>,
        dest: Destination,
    ) -> Result<(), LocalRepositoryError> {
        for i in data {
            self.save_row_order_in_file(i, dest.clone()).await;
        }
        Ok(())
    }
}
