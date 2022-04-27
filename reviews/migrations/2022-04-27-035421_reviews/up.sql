-- Your SQL goes here
CREATE TABLE IF NOT EXISTS review (
    id SERIAL PRIMARY KEY NOT NULL,
    body VARCHAR(250) NOT NULL,
    author_id INT NOT NULL,
    product_id INT NOT NULL
);