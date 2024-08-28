use std::{error::Error, future::Future, sync::Arc};

use tokio::{fs::File, io::AsyncWriteExt, sync::RwLock};

use crate::contracts::saver::Saver;

#[derive(Clone)]
pub struct DataSaver {
    file: Arc<RwLock<File>>,
}

impl DataSaver {
    pub fn new(file: File) -> Self {
        Self {
            file: Arc::new(RwLock::new(file)),
        }
    }
}

impl Saver for DataSaver {
    fn save(&self, data: String) -> impl Future<Output = Result<(), Box<dyn Error>>> {
        async {
            let save_data=data+"\n";
            let mut guard = self.file.write().await;
            guard.write_all(save_data.as_bytes()).await?;
            Ok(())
        }
    }
}
