CREATE TABLE urls (
  id          VARCHAR(21) PRIMARY KEY NOT NULL,
  created_at  TIMESTAMP NOT NULL,
  updated_at  TIMESTAMP NOT NULL,

  url         TEXT UNIQUE NOT NULL,
  title       TEXT,
  description TEXT,
  image       TEXT,
  created_by  VARCHAR(21) NOT NULL REFERENCES users(id)
);
