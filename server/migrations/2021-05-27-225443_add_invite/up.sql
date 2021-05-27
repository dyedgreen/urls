CREATE TABLE invites (
  id          VARCHAR(21) PRIMARY KEY NOT NULL,
  created_at  TIMESTAMP NOT NULL,
  updated_at  TIMESTAMP NOT NULL,

  token       TEXT        UNIQUE NOT NULL,
  created_by  VARCHAR(21) NOT NULL REFERENCES users(id),
  claimed_by  VARCHAR(21) UNIQUE REFERENCES users(id)
);
