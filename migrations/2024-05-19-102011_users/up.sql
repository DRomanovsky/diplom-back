CREATE TABLE users
(
  id          VARCHAR(255) PRIMARY KEY,
  email       VARCHAR(255) NOT NULL,
  pass        VARCHAR(255) NOT NULL,
  created_at  TIMESTAMP,
  updated_at  TIMESTAMP
)
