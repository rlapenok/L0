use std::{error::Error, fmt::Display};

use axum::{http::StatusCode, response::IntoResponse, Json};

use crate::model::responses::{AddOrderResponse, OrderStatus};

use super::remote_repository_error::RemoteRepositoryError;

#[derive(Debug)]
pub enum RemoteServiceError {
    RemoteRepositoryErrors(RemoteRepositoryError),
    SerderError(serde_json::Error),
}
impl Display for RemoteServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RemoteRepositoryErrors(err) => err.fmt(f),
            Self::SerderError(err) => err.fmt(f),
        }
    }
}
impl Error for RemoteServiceError {}

impl IntoResponse for RemoteServiceError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::RemoteRepositoryErrors(err) => match err {
                RemoteRepositoryError::PostgresErrors(err) => {
                    if let Some(err) = err.as_database_error() {
                        let body = AddOrderResponse::new(
                            Some(err.to_string()),
                            None,
                            None,
                            OrderStatus::NotAccepted,
                        );
                        return (StatusCode::CONFLICT, Json(body)).into_response();
                    }
                    let body = AddOrderResponse::new(
                        Some(err.to_string()),
                        None,
                        None,
                        OrderStatus::WillBeAccepted,
                    );
                    (StatusCode::MULTI_STATUS, Json(body)).into_response()
                }
                RemoteRepositoryError::RedisErrors(err) => {
                    let body = AddOrderResponse::new(
                        None,
                        Some(err.to_string()),
                        None,
                        OrderStatus::WillBeAccepted,
                    );
                    (StatusCode::MULTI_STATUS, Json(body)).into_response()
                }
                RemoteRepositoryError::RedisUniqueErrorAndPosgresOk => {
                    let body = AddOrderResponse::new(
                        None,
                        Some("This order_uid exists in RadisDb".to_owned()),
                        None,
                        OrderStatus::Accepted,
                    );
                    (StatusCode::MULTI_STATUS, Json(body)).into_response()
                }
            },
            Self::SerderError(err) => {
                let body = AddOrderResponse::new(
                    None,
                    None,
                    Some(err.to_string()),
                    OrderStatus::NotAccepted,
                );
                (StatusCode::INTERNAL_SERVER_ERROR, Json(body)).into_response()
            }
        }
    }
}
impl From<serde_json::Error> for RemoteServiceError {
    fn from(value: serde_json::Error) -> Self {
        Self::SerderError(value)
    }
}

impl From<RemoteRepositoryError> for RemoteServiceError {
    fn from(value: RemoteRepositoryError) -> Self {
        Self::RemoteRepositoryErrors(value)
    }
}
