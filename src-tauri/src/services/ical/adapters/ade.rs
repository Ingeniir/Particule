use crate::models::schedule::CourseKind;
use crate::services::ical::{NormalizedEvent, parser::RawEvent};
use regex::Regex;
use std::sync::LazyLock;

/// CC, CC1, Contrôle, Examen, Partiel, DS, Évaluation, Soutenance
static EXAM_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(?:CC\s*\d*|contr[ôo]le|examen|partiel|DS\s*\d*|[ée]valuation|soutenance)\b")
        .unwrap()
});

/// CM / TD / TP, éventuellement suivis d'un numéro de groupe : "TD 1", "TP1B", "TD1"
static TYPE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\b(CM|TD|TP)\s*(\d+[A-Za-z]?)?\b").unwrap());

pub fn to_event(raw: &RawEvent) -> NormalizedEvent {
    let (teachers, groups) = split_description(&raw.description);

    let mut kind = None;
    let mut group_label = None;
    let mut title = raw.summary.clone();

    // 1 & 2. Le titre (SUMMARY) est prioritaire
    if let Some(m) = EXAM_RE.find(&raw.summary) {
        kind = Some(CourseKind::Cc);
        title = strip(&raw.summary, m.range());
    } else if let Some(c) = TYPE_RE.captures(&raw.summary) {
        let whole = c.get(0).unwrap();
        kind = Some(kind_from(&c[1]));
        if c.get(2).is_some() {
            group_label = Some(whole.as_str().trim().to_string());
        }
        title = strip(&raw.summary, whole.range());
    }

    // 3. Sinon, la description (lignes de groupes)
    if kind.is_none() && groups.iter().any(|g| EXAM_RE.is_match(g)) {
        kind = Some(CourseKind::Cc);
    }
    if kind.is_none() {
        kind = kind_from_groups(&groups);
    }

    // 4. Sinon, la salle
    if kind.is_none() && raw.location.to_lowercase().contains("amphi") {
        kind = Some(CourseKind::Cm);
    }

    NormalizedEvent {
        uid: raw.uid.clone(),
        title,
        raw_summary: raw.summary.clone(),
        kind: kind.unwrap_or(CourseKind::Other),
        group_label,
        teachers,
        groups,
        location: (!raw.location.is_empty()).then(|| raw.location.clone()),
        description: raw.description.clone(),
        start: raw.start,
        end: raw.end,
    }
}

fn kind_from(code: &str) -> CourseKind {
    match code {
        "CM" => CourseKind::Cm,
        "TD" => CourseKind::Td,
        "TP" => CourseKind::Tp,
        _ => CourseKind::Other,
    }
}

/// Plusieurs promos dans un même créneau : on garde le type le plus spécifique.
fn kind_from_groups(groups: &[String]) -> Option<CourseKind> {
    let found: Vec<CourseKind> = groups
        .iter()
        .filter_map(|g| TYPE_RE.captures(g).map(|c| kind_from(&c[1])))
        .collect();
    [CourseKind::Tp, CourseKind::Td, CourseKind::Cm]
        .into_iter()
        .find(|k| found.contains(k))
}

/// Retire le marqueur de type du titre : "Commerce international - TD 1" → "Commerce international"
fn strip(s: &str, r: std::ops::Range<usize>) -> String {
    let t = format!("{}{}", &s[..r.start], &s[r.end..]);
    let t = t
        .trim_matches(|c: char| c == '-' || c == '–' || c == ':' || c.is_whitespace())
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    if t.is_empty() {
        s.trim().to_string()
    } else {
        t
    }
}

/// Sépare les lignes de DESCRIPTION en (enseignants, groupes).
fn split_description(desc: &str) -> (Vec<String>, Vec<String>) {
    let mut teachers = Vec::new();
    let mut groups = Vec::new();
    for line in desc.lines().map(str::trim).filter(|l| !l.is_empty()) {
        if line.starts_with("(Export") {
            continue;
        }
        if looks_like_person(line) {
            teachers.push(line.to_string());
        } else {
            groups.push(line.to_string());
        }
    }
    (teachers, groups)
}

/// ADE écrit les enseignants "NOM PRÉNOM" en majuscules, sans chiffres.
fn looks_like_person(line: &str) -> bool {
    let words = line.split_whitespace().count();
    (2..=5).contains(&words)
        && !line.chars().any(|c| c.is_ascii_digit())
        && !line.contains(" - ")
        && line
            .chars()
            .filter(|c| c.is_alphabetic())
            .all(|c| c.is_uppercase())
        && !TYPE_RE.is_match(line)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::ical::parser;

    fn events() -> Vec<NormalizedEvent> {
        let ics = include_str!("../../../../tests/fixtures/ade_reunion.ics");
        parser::parse(ics, chrono_tz::Indian::Reunion)
            .events
            .iter()
            .map(to_event)
            .collect()
    }

    #[test]
    fn td_from_summary_with_teacher() {
        let e = events()
            .into_iter()
            .find(|e| e.raw_summary.starts_with("Commerce"))
            .unwrap();
        assert_eq!(e.kind, CourseKind::Td);
        assert_eq!(e.title, "Commerce international");
        assert_eq!(e.group_label.as_deref(), Some("TD 1"));
        assert_eq!(e.teachers, vec!["SVIZZERO SERGE"]);
        assert!(e.groups.contains(&"TD CPES 3 ECONOMIE".to_string()));
    }

    #[test]
    fn tp_group_without_teacher() {
        let e = events()
            .into_iter()
            .find(|e| e.raw_summary.ends_with("TP1B"))
            .unwrap();
        assert_eq!(e.kind, CourseKind::Tp);
        assert_eq!(e.title, "UEO L3 MIASHS");
        assert_eq!(e.group_label.as_deref(), Some("TP1B"));
        assert!(e.teachers.is_empty());
    }

    #[test]
    fn cm_and_td_from_description() {
        let all = events();
        let amphi = all
            .iter()
            .find(|e| e.location.as_deref() == Some("Amphi C"))
            .unwrap();
        assert_eq!(amphi.kind, CourseKind::Cm);
        assert_eq!(amphi.teachers, vec!["SAINTIER RENAUD"]);

        let td = all
            .iter()
            .find(|e| e.location.as_deref() == Some("S1-1-S1") && e.title == "Topologie")
            .unwrap();
        assert_eq!(td.kind, CourseKind::Td);
    }

    #[test]
    fn cc_detected() {
        let raw = RawEvent {
            uid: None,
            summary: "Topologie - CC1".into(),
            description: String::new(),
            location: String::new(),
            start: chrono::Utc::now(),
            end: chrono::Utc::now() + chrono::Duration::hours(2),
        };
        let e = to_event(&raw);
        assert_eq!(e.kind, CourseKind::Cc);
        assert_eq!(e.title, "Topologie");
    }
}
