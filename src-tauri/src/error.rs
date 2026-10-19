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
    #[error("Erreur réseau : {0}")]
    Http(String),
}

impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::RowNotFound => AppError::NotFound,
            other => AppError::Database(other.to_string()),
        }
    }
}

impl From<reqwest::Error> for AppError {
    fn from(e: reqwest::Error) -> Self {
        if e.is_timeout() {
            AppError::Http("Le serveur de l'université ne répond pas (délai dépassé)".into())
        } else {
            AppError::Http(e.to_string())
        }
    }
}

pub type AppResult<T> = Result<T, AppError>;
