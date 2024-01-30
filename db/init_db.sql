-- For testing, just drop & recreate on startup
DROP TABLE IF EXISTS products;

CREATE TABLE IF NOT EXISTS products (
    ID SERIAL,
    Name VARCHAR(255),
    Store VARCHAR(100),
    SearchStr VARCHAR(255),
    Price FLOAT4
);

CREATE EXTENSION pg_trgm;

CREATE INDEX idx_product_name ON products USING gin(to_tsvector('dutch', SearchStr));