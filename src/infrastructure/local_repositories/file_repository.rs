use std::{collections::VecDeque, sync::Arc};

use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
    sync::{Mutex, MutexGuard},
};
use tracing::{error, instrument, trace};

use crate::{
    domain::{
        local_repositories::in_file_order_presentation_repository::{
            InFileOrderPresentationRepository, PostgresRawDataFromFile, RedisRawDataInFromFile,
        },
        models::Destination,
    },
    errors::local_repository_error::LocalRepositoryError,
};
//repository for save raw order in files
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
    //method for getting file guard
    async fn get_guard(&self, dest: Destination) -> MutexGuard<'_, File> {
        match dest {
            Destination::Postgres => self.file_postgres.lock().await,
            Destination::Redis => self.file_redis.lock().await,
        }
    }
    #[instrument(
        skip(self, data, guard),
        name = "FileRepository::save_raw_order_in_file"
    )]
    async fn save_raw_order_in_file(&self, data: String, mut guard: MutexGuard<'_, File>) {
        trace!("Start save raw orders in file");
        //add delimiter for raw order
        let save_data = data + "\n#\n";
        //save raw order in file
        if let Err(err) = guard.write_all(save_data.as_bytes()).await {
            error!("Error when saving a raw order to a file:{}", err)
        }
        //flush buffer
        if let Err(err) = guard.flush().await {
            error!("Error when saving a raw order to a file:{}", err)
        }
        trace!("Raw order was saved")
    }

    #[instrument(skip(self, dest), name = "FileRepository::read_row_data_from_files")]
    async fn read_row_data_from_files(
        &self,
        dest: Destination,
    ) -> Result<Option<VecDeque<String>>, LocalRepositoryError> {
        trace!("Start read raw orders in file");
        //get guard for file
        let mut guard = self.get_guard(dest).await;
        let mut buff = String::new();
        //read file
        guard
            .read_to_string(&mut buff)
            .await
            .inspect_err(|err| trace!("Error reading file:{}", err))?;
        //collect !empty strings
        let data = buff
            .split("\n#\n")
            .filter(|data| !data.is_empty())
            .map(|data| data.to_owned())
            .collect::<VecDeque<String>>();
        //cleaning file
        guard
            .set_len(0)
            .await
            .inspect_err(|err| error!("Error reading file:{}", err))?;
        trace!("Orders previously unprocessed by the server were read");
        Ok(if data.is_empty() { None } else { Some(data) })
    }
}

impl InFileOrderPresentationRepository for FileRepository {
    #[instrument(skip(self), name = "InFileOrderPresentationRepository::get_raw_orders")]
    async fn get_raw_orders(
        &self,
    ) -> Result<(PostgresRawDataFromFile, RedisRawDataInFromFile), LocalRepositoryError> {
        //reading raw orders from files
        trace!("Starting read raw orders from files");
        let postgres_row_data = self
            .read_row_data_from_files(Destination::Postgres)
            .await
            .inspect_err(|err| trace!("Error reading file with postgres orders :{}", err))?;
        let redis_row_data = self
            .read_row_data_from_files(Destination::Redis)
            .await
            .inspect_err(|err| trace!("Error reading file with redis orders :{}", err))?;
        trace!("Raw orders by server were read");
        Ok((postgres_row_data, redis_row_data))
    }
    #[instrument(
        skip(self, raw_orders, dest),
        name = "InFileOrderPresentationRepository::save_orders"
    )]
    async fn save_orders(&self, raw_orders: VecDeque<String>, dest: Destination) {
        trace!("Start save raw orders");
        //save each row order
        for i in raw_orders {
            //get guard
            let guard = self.get_guard(dest.clone()).await;
            //save raw order in file
            self.save_raw_order_in_file(i, guard).await;
        }
        trace!("Raw orders was saved")
    }
}
