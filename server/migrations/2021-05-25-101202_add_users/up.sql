CREATE TABLE users (
  id          VARCHAR(21) PRIMARY KEY NOT NULL,
  created_at  TIMESTAMP NOT NULL,
  updated_at  TIMESTAMP NOT NULL,

  name  TEXT  NOT NULL,
  email TEXT  UNIQUE NOT NULL
);
