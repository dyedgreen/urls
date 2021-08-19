DROP TABLE logins;

CREATE TABLE logins (
  id          VARCHAR(21) PRIMARY KEY NOT NULL,
  created_at  TIMESTAMP NOT NULL,
  updated_at  TIMESTAMP NOT NULL,

  user_id    VARCHAR(21) NOT NULL REFERENCES users(id),
  token       TEXT NOT NULL,
  valid_until TIMESTAMP NOT NULL,
  claimed     BOOLEAN NOT NULL
);
