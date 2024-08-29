use std::{error::Error, future::Future, sync::Arc};

use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
    sync::Mutex,
};

use crate::contracts::saver::WriterReader;

#[derive(Clone)]
pub struct DataWriterReader {
    file: Arc<Mutex<File>>,
}

impl DataWriterReader {
    pub fn new(file: File) -> Self {
        Self {
            file: Arc::new(Mutex::new(file)),
        }
    }
}

impl WriterReader for DataWriterReader {
    fn write(&self, data: String) -> impl Future<Output = Result<(), Box<dyn Error>>> {
        async {
            let save_data = data + "\n#\n";
            let mut guard = self.file.lock().await;
            guard.write_all(save_data.as_bytes()).await?;
            guard.flush().await?;
            Ok(())
        }
    }
    fn read(&self) -> impl Future<Output = Result<Option<Vec<String>>, Box<dyn Error>>> {
        async move {
            let mut buff = String::new();
            let mut guard = self.file.lock().await;
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
    }
}
