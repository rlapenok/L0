use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum RemoteRepositoryError {
    PostgresRepositoryErrors(sqlx::Error),
    RedisRepositoryErrors(deadpool_redis::PoolError),
}
impl Display for RemoteRepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PostgresRepositoryErrors(err) => err.fmt(f),
            Self::RedisRepositoryErrors(err) => err.fmt(f),
        }
    }
}
impl Error for RemoteRepositoryError {}
impl From<sqlx::Error> for RemoteRepositoryError {
    fn from(value: sqlx::Error) -> Self {
        Self::PostgresRepositoryErrors(value)
    }
}
impl From<deadpool_redis::PoolError> for RemoteRepositoryError {
    fn from(value: deadpool_redis::PoolError) -> Self {
        Self::RedisRepositoryErrors(value)
    }
}

impl RemoteRepositoryError{
    pub fn is_unique_postgres_err(&self)->bool{
        match self {
            Self::PostgresRepositoryErrors(err)=>{
                match err {
                    sqlx::Error::Database(err)=>{
                        err.is_unique_violation()
                    }
                    _=>false
                }
            }
            _=>false
        }
    }
}
