use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use chrono_tz::Tz;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct RawEvent {
    pub uid: Option<String>,
    pub summary: String,
    pub description: String,
    pub location: String,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

#[derive(Debug, Default)]
pub struct ParseOutput {
    pub events: Vec<RawEvent>,
    pub warnings: Vec<String>,
}

struct Prop {
    tzid: Option<String>,
    value: String,
}

/// RFC 5545 §3.1 : une ligne qui commence par un espace/tab prolonge la précédente.
pub fn unfold(input: &str) -> String {
    let normalized = input.replace("\r\n", "\n").replace('\r', "\n");
    let mut out = String::with_capacity(normalized.len());
    for line in normalized.split('\n') {
        if let Some(rest) = line.strip_prefix(' ').or_else(|| line.strip_prefix('\t')) {
            out.push_str(rest);
        } else {
            if !out.is_empty() {
                out.push('\n');
            }
            out.push_str(line);
        }
    }
    out
}

/// Décode les échappements TEXT : \n \, \; \\
pub fn unescape(v: &str) -> String {
    let mut out = String::with_capacity(v.len());
    let mut chars = v.chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('n') | Some('N') => out.push('\n'),
                Some(other) => out.push(other),
                None => {}
            }
        } else {
            out.push(c);
        }
    }
    out
}

pub fn parse(input: &str, fallback_tz: Tz) -> ParseOutput {
    let mut out = ParseOutput::default();
    let unfolded = unfold(input);
    let mut current: Option<HashMap<String, Prop>> = None;
    let mut nested = 0usize; // VALARM etc. à l'intérieur d'un VEVENT

    for line in unfolded.lines() {
        let line = line.trim_end();
        if line.is_empty() {
            continue;
        }
        let Some((name, params, value)) = split_line(line) else {
            continue;
        };

        match name.as_str() {
            "BEGIN" if value.eq_ignore_ascii_case("VEVENT") => {
                current = Some(HashMap::new());
                nested = 0;
            }
            "BEGIN" if current.is_some() => nested += 1,
            "END" if value.eq_ignore_ascii_case("VEVENT") => {
                if let Some(props) = current.take() {
                    match build_event(&props, fallback_tz) {
                        Ok(ev) => out.events.push(ev),
                        Err(w) => out.warnings.push(w),
                    }
                }
            }
            "END" if nested > 0 => nested -= 1,
            _ => {
                if nested == 0 {
                    if let Some(props) = current.as_mut() {
                        let tzid = param(&params, "TZID");
                        props.entry(name.clone()).or_insert(Prop {
                            tzid,
                            value: value.to_string(),
                        });
                    }
                }
            }
        }
    }

    if current.is_some() {
        out.warnings
            .push("Dernier événement incomplet (END:VEVENT manquant), ignoré".into());
    }
    out
}

fn split_line(line: &str) -> Option<(String, Vec<String>, &str)> {
    let mut in_quotes = false;
    let mut colon = None;
    for (i, c) in line.char_indices() {
        match c {
            '"' => in_quotes = !in_quotes,
            ':' if !in_quotes => {
                colon = Some(i);
                break;
            }
            _ => {}
        }
    }
    let colon = colon?;
    let (head, value) = (&line[..colon], &line[colon + 1..]);
    let mut parts = head.split(';');
    let name = parts.next()?.trim().to_ascii_uppercase();
    let params = parts.map(str::to_string).collect();
    Some((name, params, value))
}

fn param(params: &[String], key: &str) -> Option<String> {
    params.iter().find_map(|p| {
        let (k, v) = p.split_once('=')?;
        k.eq_ignore_ascii_case(key)
            .then(|| v.trim_matches('"').to_string())
    })
}

/// Gère : UTC (…Z), heure locale avec TZID, heure "flottante" (fuseau par défaut).
fn parse_dt(value: &str, tzid: Option<&str>, fallback: Tz) -> Option<DateTime<Utc>> {
    let v = value.trim();
    if let Some(s) = v.strip_suffix('Z') {
        return NaiveDateTime::parse_from_str(s, "%Y%m%dT%H%M%S")
            .ok()
            .map(|n| n.and_utc());
    }
    let naive = NaiveDateTime::parse_from_str(v, "%Y%m%dT%H%M%S").ok()?;
    let tz = tzid.and_then(|t| t.parse::<Tz>().ok()).unwrap_or(fallback);
    tz.from_local_datetime(&naive)
        .earliest()
        .map(|d| d.with_timezone(&Utc))
}

fn build_event(props: &HashMap<String, Prop>, tz: Tz) -> Result<RawEvent, String> {
    let text = |k: &str| props.get(k).map(|p| unescape(&p.value)).unwrap_or_default();
    let summary = text("SUMMARY").trim().to_string();

    let sp = props
        .get("DTSTART")
        .ok_or_else(|| format!("« {summary} » : DTSTART manquant"))?;
    let start = parse_dt(&sp.value, sp.tzid.as_deref(), tz)
        .ok_or_else(|| format!("« {summary} » : date de début illisible ({})", sp.value))?;
    let end = props
        .get("DTEND")
        .and_then(|p| parse_dt(&p.value, p.tzid.as_deref(), tz))
        .unwrap_or(start + chrono::Duration::hours(1));
    if end <= start {
        return Err(format!("« {summary} » : fin avant le début, ignoré"));
    }

    Ok(RawEvent {
        uid: props.get("UID").map(|p| p.value.trim().to_string()),
        summary,
        description: text("DESCRIPTION"),
        location: text("LOCATION").trim().to_string(),
        start,
        end,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    const ADE: &str = include_str!("../../../tests/fixtures/ade_reunion.ics");

    #[test]
    fn unfold_joins_continuation_lines() {
        assert_eq!(
            unfold("DESCRIPTION:abc\r\n def\r\nUID:1"),
            "DESCRIPTION:abcdef\nUID:1"
        );
    }

    #[test]
    fn parses_ade_export_and_tolerates_truncated_event() {
        let out = parse(ADE, chrono_tz::Indian::Reunion);
        assert_eq!(out.events.len(), 7);
        assert_eq!(out.warnings.len(), 1);

        let topo = out.events.iter().find(|e| e.location == "Amphi C").unwrap();
        assert_eq!(topo.summary, "Topologie");
        assert_eq!(topo.start.to_rfc3339(), "2026-10-22T10:00:00+00:00");
        assert!(topo.description.contains("SAINTIER RENAUD"));
        assert!(topo.description.contains("(Exported :19/10/2026")); // ligne repliée recollée
    }
}
