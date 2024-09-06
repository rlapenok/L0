use std::{error::Error, fmt::Display};

//errors for local repository
#[derive(Debug)]
pub enum LocalRepositoryError {
    Error(std::io::Error),
}
impl Display for LocalRepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Error(err) => err.fmt(f),
        }
    }
}
impl Error for LocalRepositoryError {}

impl From<std::io::Error> for LocalRepositoryError {
    fn from(value: std::io::Error) -> Self {
        Self::Error(value)
    }
}
