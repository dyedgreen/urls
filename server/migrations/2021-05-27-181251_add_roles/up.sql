CREATE TABLE roles (
  id          VARCHAR(21) PRIMARY KEY NOT NULL,
  created_at  TIMESTAMP NOT NULL,
  updated_at  TIMESTAMP NOT NULL,

  user_id     VARCHAR(21) NOT NULL REFERENCES users(id),
  permission  TEXT        NOT NULL,
  UNIQUE (user_id, permission)
);
