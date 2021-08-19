DROP TABLE logins;

CREATE TABLE logins (
  id          VARCHAR(21) PRIMARY KEY NOT NULL,
  created_at  TIMESTAMP NOT NULL,
  updated_at  TIMESTAMP NOT NULL,

  user_id         VARCHAR(21) NOT NULL REFERENCES users(id),
  email_token     TEXT NOT NULL,
  claim_until     TIMESTAMP NOT NULL,
  claimed         BOOLEAN NOT NULL,
  session_token   TEXT UNIQUE,
  last_used       TIMESTAMP NOT NULL,
  last_user_agent TEXT,
  revoked         BOOLEAN NOT NULL
);
