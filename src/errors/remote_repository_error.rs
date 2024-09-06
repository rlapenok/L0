use std::{error::Error, fmt::Display};

//errors for remote repository
#[derive(Debug)]
pub enum RemoteRepositoryError {
    PostgresErrors(sqlx::Error),
    RedisErrors(deadpool_redis::PoolError),
    RedisUniqueErrorAndPosgresOk,
}
impl RemoteRepositoryError {
    //method for check postgres unique error
    pub fn is_unique_postgres_err(&self) -> bool {
        match self {
            Self::PostgresErrors(err) => {
                if let Some(err) = err.as_database_error() {
                    return err.is_unique_violation();
                }
                false
            }
            _ => false,
        }
    }
    //method for check redis unique error
    pub fn is_unique_redis_err(&self) -> bool {
        matches!(self, RemoteRepositoryError::RedisUniqueErrorAndPosgresOk)
    }
}

pub enum RemoteRepositoryResponse<T> {
    OrderFromRedis(String),
    OrderFromPostgres(T, String),
}

impl Display for RemoteRepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PostgresErrors(err) => err.fmt(f),
            Self::RedisErrors(err) => err.fmt(f),
            Self::RedisUniqueErrorAndPosgresOk => {
                writeln!(f, "This order_uid exists in Radis")
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
