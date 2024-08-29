use std::{error::Error, future::Future};

use confique::Config;
use tokio::fs::File;

use crate::{contracts::saver::ToSaver, data_saver::data_saver::DataWriterReader};

#[derive(Config)]
pub struct DataSaverConfig {
    path: String,
}

impl ToSaver for DataSaverConfig {
    type Output = DataWriterReader;
    fn to_saver(&self) -> impl Future<Output = Result<Self::Output, Box<dyn Error>>> {
        async {
            let file = File::options()
                .append(true)
                .read(true)
                .open(&self.path)
                .await?;
            Ok(DataWriterReader::new(file))
        }
    }
}
