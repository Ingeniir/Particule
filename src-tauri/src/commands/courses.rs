// courses.rs
use crate::{
    db::Db,
    error::{AppError, AppResult},
    models::course::{Course, NewCourse},
    repositories::course_repo,
};
use tauri::State;

#[tauri::command]
#[specta::specta]
pub async fn list_courses(db: State<'_, Db>) -> AppResult<Vec<Course>> {
    Ok(course_repo::list(&db.0).await?)
}

#[tauri::command]
#[specta::specta]
pub async fn create_course(db: State<'_, Db>, input: NewCourse) -> AppResult<Course> {
    if input.name.trim().is_empty() {
        return Err(AppError::Validation("Le nom du cours est requis".into()));
    }
    let id = uuid::Uuid::now_v7().to_string();
    Ok(course_repo::insert(&db.0, &id, &input).await?)
}

#[tauri::command]
#[specta::specta]
pub async fn delete_course(db: State<'_, Db>, id: String) -> AppResult<()> {
    Ok(course_repo::delete(&db.0, &id).await?)
}
