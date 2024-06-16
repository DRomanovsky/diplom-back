CREATE TABLE services_categories
(
  id          VARCHAR(255) PRIMARY KEY,
  name        VARCHAR(255) NOT NULL,
  imagePath   VARCHAR,
  created_at  TIMESTAMP,
  updated_at  TIMESTAMP
)