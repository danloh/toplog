-- Your SQL goes here

CREATE TABLE background_jobs (
  id BIGSERIAL PRIMARY KEY,
  job_type TEXT NOT NULL,
  data JSONB NOT NULL,
  retries INTEGER NOT NULL DEFAULT 0,
  last_retry TIMESTAMP NOT NULL DEFAULT '1970-01-01',
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE items ALTER COLUMN is_top SET DEFAULT FALSE;