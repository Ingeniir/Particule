use serde::Serialize;

#[derive(Debug, thiserror::Error, Serialize, specta::Type)]
#[serde(tag = "kind", content = "message")]
pub enum AppError {
    #[error("Erreur base de données : {0}")]
    Db(String),
    #[error("{0}")]
    Validation(String),
}

impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        Self::Db(e.to_string())
    }
}

pub type AppResult<T> = Result<T, AppError>;
