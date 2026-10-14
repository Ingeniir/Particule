CREATE TABLE semester (
  id          TEXT PRIMARY KEY,
  name        TEXT NOT NULL,
  start_date  TEXT NOT NULL,
  end_date    TEXT NOT NULL,
  created_at  TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now'))
);

CREATE TABLE course (
  id           TEXT PRIMARY KEY,
  semester_id  TEXT REFERENCES semester(id) ON DELETE SET NULL,
  code         TEXT,
  name         TEXT NOT NULL,
  color        TEXT NOT NULL DEFAULT '#6366f1',
  teacher      TEXT,
  created_at   TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now'))
);

CREATE TABLE task (
  id             TEXT PRIMARY KEY,
  course_id      TEXT REFERENCES course(id) ON DELETE SET NULL,
  title          TEXT NOT NULL,
  notes          TEXT,
  due_at         TEXT,
  status         TEXT NOT NULL DEFAULT 'todo' CHECK (status IN ('todo','doing','done')),
  priority       INTEGER NOT NULL DEFAULT 2,
  estimated_min  INTEGER,
  created_at     TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  completed_at   TEXT
);

CREATE TABLE pomodoro_session (
  id          TEXT PRIMARY KEY,
  task_id     TEXT REFERENCES task(id) ON DELETE SET NULL,
  course_id   TEXT REFERENCES course(id) ON DELETE SET NULL,
  started_at  TEXT NOT NULL,
  ended_at    TEXT,
  work_min    INTEGER NOT NULL,
  completed   INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX idx_task_course ON task(course_id);
CREATE INDEX idx_task_due ON task(due_at);
