use crate::ClipError;
use crate::DataError;
pub mod ask;
pub mod action;

#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("Clip Error {0}")]
    Clip(#[from] ClipError),
    #[error("Database Error {0}")]
    Data(DataError),
    #[error("Not found")]
    NotFound,
    #[error("Permission Error {0}")]
    PermissionError(String)
}

impl From<DataError> for ServiceError {
    fn from(err: DataError) -> Self {
        match err {
                DataError::Database(d) => match d {
                    sqlx::Error::RowNotFound => Self::NotFound,
                    other => Self::Data(DataError::Database(other))
                }
        }
    }
}

impl From<sqlx::Error> for ServiceError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => Self::NotFound,
            other => Self::Data(DataError::Database(other))
        }
    }
}