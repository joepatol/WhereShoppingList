-- For testing, just drop & recreate on startup
DROP TABLE IF EXISTS products;

CREATE TABLE IF NOT EXISTS products (
    ID SERIAL,
    Name VARCHAR(255),
    Store VARCHAR(100),
    SearchStr VARCHAR(255),
    Price FLOAT4
);