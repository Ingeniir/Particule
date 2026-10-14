// course_repo.rs
use crate::models::course::{Course, NewCourse};
use sqlx::SqlitePool;

pub async fn list(pool: &SqlitePool) -> Result<Vec<Course>, sqlx::Error> {
    sqlx::query_as::<_, Course>(
        "SELECT id, semester_id, code, name, color, teacher, created_at FROM course ORDER BY name",
    )
    .fetch_all(pool)
    .await
}

pub async fn insert(pool: &SqlitePool, id: &str, c: &NewCourse) -> Result<Course, sqlx::Error> {
    sqlx::query_as::<_, Course>(
        "INSERT INTO course (id, code, name, color, teacher) VALUES (?, ?, ?, ?, ?) \
         RETURNING id, semester_id, code, name, color, teacher, created_at",
    )
    .bind(id)
    .bind(&c.code)
    .bind(c.name.trim())
    .bind(&c.color)
    .bind(&c.teacher)
    .fetch_one(pool)
    .await
}

pub async fn delete(pool: &SqlitePool, id: &str) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM course WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}
