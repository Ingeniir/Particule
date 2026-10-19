use crate::models::schedule::{CourseKind, ScheduleEvent};
use crate::services::ical::NormalizedEvent;
use chrono::{DateTime, SecondsFormat, Utc};
use sqlx::SqlitePool;

/// Format unique en base → les comparaisons de chaînes = comparaisons de dates.
pub fn iso(d: DateTime<Utc>) -> String {
    d.to_rfc3339_opts(SecondsFormat::Secs, true)
}

#[derive(sqlx::FromRow)]
struct EventRow {
    id: String,
    uid: Option<String>,
    title: String,
    raw_summary: String,
    kind: String,
    group_label: Option<String>,
    teachers_json: String,
    groups_json: String,
    location: Option<String>,
    start_utc: String,
    end_utc: String,
}

impl From<EventRow> for ScheduleEvent {
    fn from(r: EventRow) -> Self {
        Self {
            id: r.id,
            uid: r.uid,
            title: r.title,
            raw_summary: r.raw_summary,
            kind: CourseKind::from_db(&r.kind),
            group_label: r.group_label,
            teachers: serde_json::from_str(&r.teachers_json).unwrap_or_default(),
            groups: serde_json::from_str(&r.groups_json).unwrap_or_default(),
            location: r.location,
            start_utc: r.start_utc,
            end_utc: r.end_utc,
        }
    }
}

/// Remplace uniquement la fenêtre couverte par l'export (l'historique antérieur est conservé).
pub async fn replace_window(
    pool: &SqlitePool,
    events: &[NormalizedEvent],
    synced_at: &str,
) -> Result<u32, sqlx::Error> {
    let (Some(from), Some(to)) = (
        events.iter().map(|e| e.start).min(),
        events.iter().map(|e| e.start).max(),
    ) else {
        return Ok(0);
    };

    let mut tx = pool.begin().await?;
    sqlx::query("DELETE FROM schedule_event WHERE start_utc >= ? AND start_utc <= ?")
        .bind(iso(from))
        .bind(iso(to))
        .execute(&mut *tx)
        .await?;

    for e in events {
        sqlx::query(
            "INSERT INTO schedule_event
             (id, uid, title, raw_summary, kind, group_label, teachers_json, groups_json,
              location, description, start_utc, end_utc, synced_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(uuid::Uuid::now_v7().to_string())
        .bind(&e.uid)
        .bind(&e.title)
        .bind(&e.raw_summary)
        .bind(e.kind.as_str())
        .bind(&e.group_label)
        .bind(serde_json::to_string(&e.teachers).unwrap_or_else(|_| "[]".into()))
        .bind(serde_json::to_string(&e.groups).unwrap_or_else(|_| "[]".into()))
        .bind(&e.location)
        .bind(&e.description)
        .bind(iso(e.start))
        .bind(iso(e.end))
        .bind(synced_at)
        .execute(&mut *tx)
        .await?;
    }
    tx.commit().await?;
    Ok(events.len() as u32)
}

/// Événements qui chevauchent [from, to[.
pub async fn list_between(
    pool: &SqlitePool,
    from: &str,
    to: &str,
) -> Result<Vec<ScheduleEvent>, sqlx::Error> {
    let rows = sqlx::query_as::<_, EventRow>(
        "SELECT id, uid, title, raw_summary, kind, group_label, teachers_json, groups_json,
                location, start_utc, end_utc
         FROM schedule_event
         WHERE start_utc < ? AND end_utc > ?
         ORDER BY start_utc",
    )
    .bind(to)
    .bind(from)
    .fetch_all(pool)
    .await?;
    Ok(rows.into_iter().map(Into::into).collect())
}
