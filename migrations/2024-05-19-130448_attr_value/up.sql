CREATE TABLE attr_value (
    attr_id VARCHAR(255) NOT NULL REFERENCES attr(id),
    product_id VARCHAR(255) NOT NULL REFERENCES products(id),
    value VARCHAR(255),
    created_at  TIMESTAMP,
    updated_at  TIMESTAMP,
    PRIMARY KEY (attr_id, product_id)
)