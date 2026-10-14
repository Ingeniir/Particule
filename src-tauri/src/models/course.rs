// course.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, specta::Type, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Course {
    pub id: String,
    pub semester_id: Option<String>,
    pub code: Option<String>,
    pub name: String,
    pub color: String,
    pub teacher: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct NewCourse {
    pub name: String,
    pub code: Option<String>,
    pub color: String,
    pub teacher: Option<String>,
}
