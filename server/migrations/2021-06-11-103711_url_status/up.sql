CREATE TABLE urls_status_code (
  id          VARCHAR(21) PRIMARY KEY NOT NULL,
  created_at  TIMESTAMP NOT NULL,
  updated_at  TIMESTAMP NOT NULL,

  url         TEXT UNIQUE NOT NULL,
  status_code INTEGER NOT NULL,
  title       TEXT,
  description TEXT,
  image       TEXT,
  created_by  VARCHAR(21) NOT NULL REFERENCES users(id)
);

INSERT INTO urls_status_code
SELECT id, created_at, updated_at, url, 200, title, description, image, created_by FROM urls;

PRAGMA foreign_keys = OFF;
DROP TABLE urls;
ALTER TABLE urls_status_code RENAME TO urls;
PRAGMA foreign_keys = ON;
