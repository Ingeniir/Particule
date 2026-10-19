use crate::error::{AppError, AppResult};
use crate::models::schedule::SyncReport;
use crate::repositories::{schedule_repo, setting_repo};
use crate::services::ical::{NormalizedEvent, adapters::ade, parser};
use chrono::Utc;
use sqlx::SqlitePool;
use std::time::Duration;

pub const KEY_ICS_URL: &str = "schedule.ics_url";
pub const KEY_LAST_SYNC: &str = "schedule.last_sync_at";
pub const KEY_LAST_COUNT: &str = "schedule.last_sync_count";

/// Utilisé seulement pour les heures "flottantes" (ADE exporte en UTC).
/// TODO : le rendre configurable dans les paramètres.
const DEFAULT_TZ: chrono_tz::Tz = chrono_tz::Indian::Reunion;

pub async fn sync(pool: &SqlitePool) -> AppResult<SyncReport> {
    let url = setting_repo::get(pool, KEY_ICS_URL)
        .await?
        .ok_or_else(|| AppError::Validation("Aucune URL d'emploi du temps configurée".into()))?;

    let body = fetch_ics(&url).await?;
    let parsed = parser::parse(&body, DEFAULT_TZ);

    // Garde-fou : ADE renvoie parfois un calendrier vide en cas de souci → on n'efface rien.
    if parsed.events.is_empty() {
        return Err(AppError::Validation(
            "Le calendrier reçu ne contient aucun cours (URL ou ressources incorrectes ?)".into(),
        ));
    }

    let events: Vec<NormalizedEvent> = parsed.events.iter().map(ade::to_event).collect();
    let synced_at = schedule_repo::iso(Utc::now());
    let imported = schedule_repo::replace_window(pool, &events, &synced_at).await?;

    setting_repo::set(pool, KEY_LAST_SYNC, &synced_at).await?;
    setting_repo::set(pool, KEY_LAST_COUNT, &imported.to_string()).await?;

    Ok(SyncReport {
        imported,
        warnings: parsed.warnings,
        synced_at,
    })
}

async fn fetch_ics(url: &str) -> AppResult<String> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .user_agent(concat!("StudyTrack/", env!("CARGO_PKG_VERSION")))
        .build()?;
    let body = client
        .get(url)
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;
    if !body.contains("BEGIN:VCALENDAR") {
        return Err(AppError::Http(
            "La réponse n'est pas un calendrier iCal (vérifie l'URL)".into(),
        ));
    }
    Ok(body)
}
