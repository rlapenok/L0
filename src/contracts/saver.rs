use std::{error::Error, future::Future};

pub trait ToSaver {
    type Output: Saver;
    fn to_saver(&self) -> impl Future<Output = Result<Self::Output, Box<dyn Error>>>;
}

pub trait Saver {
    fn save(&self, data: String) -> impl Future<Output = Result<(), Box<dyn Error>>>;
}
