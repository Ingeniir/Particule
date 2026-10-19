use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "UPPERCASE")]
pub enum CourseKind {
    Cm,
    Td,
    Tp,
    Cc,
    Other,
}

impl CourseKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Cm => "CM",
            Self::Td => "TD",
            Self::Tp => "TP",
            Self::Cc => "CC",
            Self::Other => "OTHER",
        }
    }

    pub fn from_db(s: &str) -> Self {
        match s {
            "CM" => Self::Cm,
            "TD" => Self::Td,
            "TP" => Self::Tp,
            "CC" => Self::Cc,
            _ => Self::Other,
        }
    }
}

#[derive(Debug, Clone, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleEvent {
    pub id: String,
    pub uid: Option<String>,
    pub title: String,
    pub raw_summary: String,
    pub kind: CourseKind,
    pub group_label: Option<String>,
    pub teachers: Vec<String>,
    pub groups: Vec<String>,
    pub location: Option<String>,
    pub start_utc: String,
    pub end_utc: String,
}

#[derive(Debug, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleSettings {
    pub ics_url: Option<String>,
    pub last_sync_at: Option<String>,
    pub last_sync_count: Option<u32>,
}

#[derive(Debug, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct SyncReport {
    pub imported: u32,
    pub warnings: Vec<String>,
    pub synced_at: String,
}
