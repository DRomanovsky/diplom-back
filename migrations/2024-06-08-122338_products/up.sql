CREATE TABLE products
(
  id          VARCHAR(255) PRIMARY KEY,
  title       VARCHAR(255) NOT NULL,
  price       VARCHAR(255),
  acc         INTEGER,
  description VARCHAR(500),
  image       VARCHAR(255) NOT NULL,
  status      BOOLEAN NOT NULL,
  category_id VARCHAR(255) NOT NULL REFERENCES products_categories(id),
  created_at  TIMESTAMP,
  updated_at  TIMESTAMP
)
