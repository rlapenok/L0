use std::{error::Error, sync::Arc};

use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
    sync::{Mutex, MutexGuard},
};

use crate::{
    domain::{
        local_repositories::in_file_order_presentation_repository::InFileOrderPresentationRepository,
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
    pub fn new(files: (File, File)) -> Self {
        Self {
            file_postgres: Arc::new(Mutex::new(files.0)),
            file_redis: Arc::new(Mutex::new(files.1)),
        }
    }
    async fn get_guard(&self, files: Destination) -> MutexGuard<'_, File> {
        match files {
            Destination::PostgresFile => self.file_postgres.lock().await,
            Destination::RedisFile => self.file_redis.lock().await,
        }
    }
    async fn save_data(&self,data:String, dest:Destination)->Result<(), LocalRepositoryError>{

        let mut guard = self.get_guard(dest).await;
        let save_data = data + "\n#\n";
        guard.write_all(save_data.as_bytes()).await?;
        guard.flush().await?;
        Ok(())

    }
}

impl InFileOrderPresentationRepository for FileRepository {
    async fn get_row_data(
        &self,
        file: Destination,
    ) -> Result<Option<Vec<String>>, LocalRepositoryError> {
        let mut buff = String::new();
        let mut guard = self.file_postgres.lock().await;
        guard.read_to_string(&mut buff).await?;
        let data = buff
            .split("\n#\n")
            .filter(|data| !data.is_empty())
            .map(|data| data.to_owned())
            .collect::<Vec<String>>();
        //todo delete comment
        //guard.set_len(0).await?;
        println!("Data previously unprocessed by the server were read successfully");
        Ok(if data.is_empty() { None } else { Some(data) })
    }
    async fn save_data(&self, data: Vec<String>, dest: Destination) -> Result<(), LocalRepositoryError> {
        for i in data{
            self.save_data(i, dest.clone()).await;
        }
        Ok(())
    }
}
