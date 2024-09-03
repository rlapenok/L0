use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum RemoteRepositoryError {
    PostgresErrors(sqlx::Error),
    RedisErrors(deadpool_redis::PoolError),
    RedisUniqueErrorAndPosgresOk,
}

pub enum RemoteRepositoryResponse<T> {
    OrderFromRedis(String),
    OrderFromPostgres(T),
}

impl Display for RemoteRepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PostgresErrors(err) => err.fmt(f),
            Self::RedisErrors(err) => err.fmt(f),
            Self::RedisUniqueErrorAndPosgresOk => {
                writeln!(f, "This order_uid exists in RadisDb")
            }
        }
    }
}
impl Error for RemoteRepositoryError {}
impl From<sqlx::Error> for RemoteRepositoryError {
    fn from(value: sqlx::Error) -> Self {
        Self::PostgresErrors(value)
    }
}
impl From<deadpool_redis::PoolError> for RemoteRepositoryError {
    fn from(value: deadpool_redis::PoolError) -> Self {
        Self::RedisErrors(value)
    }
}
