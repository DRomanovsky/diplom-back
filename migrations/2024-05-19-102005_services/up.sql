CREATE TABLE services
(
  id          VARCHAR(255) PRIMARY KEY,
  title       VARCHAR(255) NOT NULL,
  price       VARCHAR(255),
  category_id VARCHAR(255) NOT NULL REFERENCES services_categories(id),
  created_at  TIMESTAMP,
  updated_at  TIMESTAMP
)
