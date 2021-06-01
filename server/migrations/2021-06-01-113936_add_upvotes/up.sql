CREATE TABLE url_upvotes (
  url_id      VARCHAR(21) NOT NULL REFERENCES urls(id),
  user_id     VARCHAR(21) NOT NULL REFERENCES users(id),
  created_at  TIMESTAMP NOT NULL,
  PRIMARY KEY (url_id, user_id)
);
