CREATE TABLE IF NOT EXISTS products (
    ID BIGSERIAL PRIMARY KEY,
    Name VARCHAR(255),
    Price FLOAT4,
    Store VARCHAR(100),
    Url VARCHAR(750),
    SearchStr VARCHAR(255)
);

CREATE EXTENSION pg_trgm;

CREATE INDEX idx_product_name ON products USING gin(to_tsvector('dutch', SearchStr));

CREATE TABLE IF NOT EXISTS scrape_errors (
    ID BIGSERIAL PRIMARY KEY,
    Scraper VARCHAR(255),
    Message TEXT
);
