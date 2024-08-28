

use crate::data_saver::data_saver::DataSaver;

#[derive(Clone)]
pub struct AppState
{
    pub saver:DataSaver,
}

impl AppState
{
    pub fn new(saver: DataSaver) -> Self {
        Self { saver }
    }
}
