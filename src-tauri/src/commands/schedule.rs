use crate::db::Db;
use crate::error::{AppError, AppResult};
use crate::models::schedule::{ScheduleEvent, ScheduleSettings, SyncReport};
use crate::repositories::{schedule_repo, setting_repo};
use crate::services::schedule_service::{self, KEY_ICS_URL, KEY_LAST_COUNT, KEY_LAST_SYNC};
use chrono::{DateTime, Utc};
use tauri::State;

#[tauri::command]
#[specta::specta]
pub async fn get_schedule_settings(db: State<'_, Db>) -> AppResult<ScheduleSettings> {
    Ok(ScheduleSettings {
        ics_url: setting_repo::get(&db.0, KEY_ICS_URL).await?,
        last_sync_at: setting_repo::get(&db.0, KEY_LAST_SYNC).await?,
        last_sync_count: setting_repo::get(&db.0, KEY_LAST_COUNT)
            .await?
            .and_then(|v| v.parse().ok()),
    })
}

#[tauri::command]
#[specta::specta]
pub async fn save_ics_url(db: State<'_, Db>, url: String) -> AppResult<()> {
    let url = url.trim();
    if url.is_empty() {
        setting_repo::delete(&db.0, KEY_ICS_URL).await?;
        return Ok(());
    }
    let url = match url.strip_prefix("webcal://") {
        Some(rest) => format!("https://{rest}"),
        None => url.to_string(),
    };
    if !(url.starts_with("https://") || url.starts_with("http://")) {
        return Err(AppError::Validation(
            "L'URL doit commencer par https://".into(),
        ));
    }
    setting_repo::set(&db.0, KEY_ICS_URL, &url).await?;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn sync_schedule(db: State<'_, Db>) -> AppResult<SyncReport> {
    schedule_service::sync(&db.0).await
}

#[tauri::command]
#[specta::specta]
pub async fn get_events(
    db: State<'_, Db>,
    from: String,
    to: String,
) -> AppResult<Vec<ScheduleEvent>> {
    let norm = |s: &str| {
        DateTime::parse_from_rfc3339(s)
            .map(|d| schedule_repo::iso(d.with_timezone(&Utc)))
            .map_err(|_| AppError::Validation(format!("Date invalide : {s}")))
    };
    Ok(schedule_repo::list_between(&db.0, &norm(&from)?, &norm(&to)?).await?)
}
