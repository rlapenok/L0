use std::{error::Error, fmt::Display};

use axum::response::IntoResponse;

use super::remote_repository_error::RemoteRepositoryError;

#[derive(Debug)]
pub enum ServiceError {
    RepositoryErrors(RemoteRepositoryError, String),
}
impl Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RepositoryErrors(err, context) => {
                writeln!(f, "Context:{}, Error description:{}", context, err)
            }
        }
    }
}
impl Error for ServiceError {}

impl IntoResponse for ServiceError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::RepositoryErrors(err, context) => {
                todo!()
            }
        }
    }
}
