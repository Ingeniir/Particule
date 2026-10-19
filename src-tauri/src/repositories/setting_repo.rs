use sqlx::SqlitePool;

pub async fn get(pool: &SqlitePool, key: &str) -> Result<Option<String>, sqlx::Error> {
    sqlx::query_scalar::<_, String>("SELECT value FROM setting WHERE key = ?")
        .bind(key)
        .fetch_optional(pool)
        .await
}

pub async fn set(pool: &SqlitePool, key: &str, value: &str) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO setting (key, value) VALUES (?, ?) ON CONFLICT(key) DO UPDATE SET value = excluded.value",
    )
    .bind(key)
    .bind(value)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn delete(pool: &SqlitePool, key: &str) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM setting WHERE key = ?")
        .bind(key)
        .execute(pool)
        .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_set_delete_roundtrip() {
        tauri::async_runtime::block_on(async {
            let pool = sqlx::sqlite::SqlitePoolOptions::new()
                .max_connections(1)
                .connect("sqlite::memory:")
                .await
                .unwrap();
            sqlx::migrate!("./migrations").run(&pool).await.unwrap();

            assert_eq!(get(&pool, "k").await.unwrap(), None);
            set(&pool, "k", "a").await.unwrap();
            set(&pool, "k", "b").await.unwrap();
            assert_eq!(get(&pool, "k").await.unwrap().as_deref(), Some("b"));
            delete(&pool, "k").await.unwrap();
            assert_eq!(get(&pool, "k").await.unwrap(), None);
        });
    }
}
