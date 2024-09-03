use std::{error::Error, fmt::Display};

use super::local_repository_error::LocalRepositoryError;

#[derive(Debug)]
pub enum LocalServiceErrors {
    LocalRepositoryError(LocalRepositoryError),
}
impl Error for LocalServiceErrors {}
impl Display for LocalServiceErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LocalRepositoryError(err) => err.fmt(f),
        }
    }
}
impl From<LocalRepositoryError> for LocalServiceErrors {
    fn from(value: LocalRepositoryError) -> Self {
        Self::LocalRepositoryError(value)
    }
}
