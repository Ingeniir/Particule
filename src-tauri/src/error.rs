use serde::Serialize;

#[derive(Debug, thiserror::Error, Serialize, specta::Type)]
#[serde(tag = "kind", content = "message")]
pub enum AppError {
    #[error("Erreur base de données : {0}")]
    Database(String),
    #[error("{0}")]
    Validation(String),
    #[error("Introuvable")]
    NotFound,
}

impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::RowNotFound => AppError::NotFound,
            other => AppError::Database(other.to_string()),
        }
    }
}

pub type AppResult<T> = Result<T, AppError>;
