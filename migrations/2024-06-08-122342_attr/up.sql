CREATE TABLE attr (
    id VARCHAR PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    category_id VARCHAR(255) NOT NULL REFERENCES products_categories(id),
    created_at  TIMESTAMP,
    updated_at  TIMESTAMP
)