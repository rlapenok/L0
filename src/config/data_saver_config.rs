use std::{error::Error, future::Future};

use confique::Config;
use tokio::fs::File;

use crate::{contracts::saver::ToSaver, data_saver::data_saver::DataSaver};

#[derive(Config)]
pub struct DataSaverConfig {
    path: String,
}

impl ToSaver for DataSaverConfig {
    type Output = DataSaver;
    fn to_saver(&self) -> impl Future<Output = Result<Self::Output, Box<dyn Error>>> {
        async {
            let file = File::create(&self.path).await?;
            Ok(DataSaver::new(file))
        }
    }
}
