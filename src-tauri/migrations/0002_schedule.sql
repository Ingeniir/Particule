CREATE TABLE setting (
  key   TEXT PRIMARY KEY,
  value TEXT NOT NULL
);

CREATE TABLE schedule_event (
  id            TEXT PRIMARY KEY,
  uid           TEXT,
  title         TEXT NOT NULL,
  raw_summary   TEXT NOT NULL,
  kind          TEXT NOT NULL CHECK (kind IN ('CM','TD','TP','CC','OTHER')),
  group_label   TEXT,
  teachers_json TEXT NOT NULL DEFAULT '[]',
  groups_json   TEXT NOT NULL DEFAULT '[]',
  location      TEXT,
  description   TEXT,          -- gardé brut pour pouvoir re-classer sans re-télécharger
  start_utc     TEXT NOT NULL,
  end_utc       TEXT NOT NULL,
  course_id     TEXT REFERENCES course(id) ON DELETE SET NULL,
  synced_at     TEXT NOT NULL
);

CREATE INDEX idx_event_start ON schedule_event(start_utc);
CREATE INDEX idx_event_end   ON schedule_event(end_utc);
