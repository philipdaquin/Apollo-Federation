-- Your SQL goes here

CREATE TABLE IF NOT EXISTS inventory (
    id SERIAL PRIMARY KEY NOT NULL,
    weight INT,
    price INT,
    in_stock BOOLEAN NOT NULL,
    shipping_estimate INT
);