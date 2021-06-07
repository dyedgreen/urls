CREATE TABLE comments (
  id          VARCHAR(21) PRIMARY KEY NOT NULL,
  created_at  TIMESTAMP NOT NULL,
  updated_at  TIMESTAMP NOT NULL,

  comment     TEXT        NOT NULL,
  url_id      VARCHAR(21) NOT NULL REFERENCES urls(id),
  created_by  VARCHAR(21) NOT NULL REFERENCES users(id),
  replies_to  VARCHAR(21) REFERENCES comments(id)
);
