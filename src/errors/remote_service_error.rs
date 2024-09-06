use std::{error::Error, fmt::Display};

use axum::{http::StatusCode, response::IntoResponse, response::Response, Json};

use crate::models::responses::{OrderResponse, OrderStatus};

use super::remote_repository_error::RemoteRepositoryError;

#[derive(Debug)]
//erros for remote service
pub enum RemoteServiceError {
    RemoteRepositoryErrors(RemoteRepositoryError),
    SerderError(serde_json::Error),
}
#[derive(Debug)]
pub enum Handler {
    AddOrder,
    GetOrder,
}

#[derive(Debug)]
pub struct RemoteServiceErrorResponse {
    handler: Handler,
    err: RemoteServiceError,
}
impl RemoteServiceErrorResponse {
    pub fn new(handler: Handler, err: RemoteServiceError) -> Self {
        Self { handler, err }
    }
}
impl Display for RemoteServiceErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.err)
    }
}
impl Error for RemoteServiceErrorResponse {}

impl IntoResponse for RemoteServiceErrorResponse {
    fn into_response(self) -> Response {
        match self.handler {
            Handler::AddOrder => match self.err {
                RemoteServiceError::RemoteRepositoryErrors(err) => match err {
                    RemoteRepositoryError::PostgresErrors(err) => {
                        if let Some(err) = err.as_database_error() {
                            let body = OrderResponse::new(
                                None,
                                Some(err.to_string()),
                                None,
                                None,
                                OrderStatus::NotAccepted,
                            );
                            (StatusCode::CONFLICT, Json(body)).into_response()
                        } else {
                            let body = OrderResponse::new(
                                None,
                                Some(err.to_string()),
                                None,
                                None,
                                OrderStatus::WillBeAccepted,
                            );
                            (StatusCode::INTERNAL_SERVER_ERROR, Json(body)).into_response()
                        }
                    }
                    RemoteRepositoryError::RedisErrors(err) => {
                        let body = OrderResponse::new(
                            None,
                            None,
                            Some(err.to_string()),
                            None,
                            OrderStatus::Accepted,
                        );
                        (StatusCode::MULTI_STATUS, Json(body)).into_response()
                    }
                    RemoteRepositoryError::RedisUniqueErrorAndPosgresOk => {
                        let body = OrderResponse::new(
                            None,
                            None,
                            Some(err.to_string()),
                            None,
                            OrderStatus::Unknown,
                        );
                        (StatusCode::MULTI_STATUS, Json(body)).into_response()
                    }
                },
                RemoteServiceError::SerderError(err) => {
                    let body = OrderResponse::new(
                        None,
                        None,
                        None,
                        Some(err.to_string()),
                        OrderStatus::NotAccepted,
                    );
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(body)).into_response()
                }
            },
            Handler::GetOrder => match self.err {
                RemoteServiceError::SerderError(err) => {
                    let body = OrderResponse::new(
                        None,
                        None,
                        None,
                        Some(err.to_string()),
                        OrderStatus::NotAccepted,
                    );
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(body)).into_response()
                }
                RemoteServiceError::RemoteRepositoryErrors(err) => match err {
                    RemoteRepositoryError::RedisUniqueErrorAndPosgresOk => {
                        unreachable!()
                    }
                    RemoteRepositoryError::PostgresErrors(err) => match err {
                        sqlx::Error::RowNotFound => {
                            let body = OrderResponse::new(
                                None,
                                Some(err.to_string()),
                                None,
                                None,
                                OrderStatus::NotAccepted,
                            );
                            (StatusCode::NOT_FOUND, Json(body)).into_response()
                        }
                        _ => {
                            let body = OrderResponse::new(
                                None,
                                Some(err.to_string()),
                                None,
                                None,
                                OrderStatus::Unknown,
                            );
                            (StatusCode::INTERNAL_SERVER_ERROR, Json(body)).into_response()
                        }
                    },
                    RemoteRepositoryError::RedisErrors(err) => {
                        let body = OrderResponse::new(
                            None,
                            None,
                            Some(err.to_string()),
                            None,
                            OrderStatus::Unknown,
                        );
                        (StatusCode::INTERNAL_SERVER_ERROR, Json(body)).into_response()
                    }
                },
            },
        }
    }
}

impl RemoteServiceError {
    //method for check postgres unique error
    pub fn is_unique_postgres_err(&self) -> bool {
        match self {
            Self::RemoteRepositoryErrors(err) => err.is_unique_postgres_err(),
            _ => false,
        }
    }
    //method for check redis unique error
    pub fn is_unique_redis_err(&self) -> bool {
        match self {
            Self::RemoteRepositoryErrors(err) => err.is_unique_redis_err(),
            _ => false,
        }
    }
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
