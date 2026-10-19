pub mod adapters;
pub mod parser;

use crate::models::schedule::CourseKind;
use chrono::{DateTime, Utc};

/// Événement normalisé et enrichi, prêt à être stocké.
#[derive(Debug, Clone)]
pub struct NormalizedEvent {
    pub uid: Option<String>,
    pub title: String,
    pub raw_summary: String,
    pub kind: CourseKind,
    pub group_label: Option<String>,
    pub teachers: Vec<String>,
    pub groups: Vec<String>,
    pub location: Option<String>,
    pub description: String,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}
