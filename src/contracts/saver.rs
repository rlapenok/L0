use std::{error::Error, future::Future};

pub trait ToSaver {
    type Output: WriterReader;
    fn to_saver(&self) -> impl Future<Output = Result<Self::Output, Box<dyn Error>>>;
}

pub trait WriterReader {
    fn write(&self, data: String) -> impl Future<Output = Result<(), Box<dyn Error>>>;
    fn read(&self) -> impl Future<Output = Result<Option<Vec<String>>, Box<dyn Error>>>;
}
