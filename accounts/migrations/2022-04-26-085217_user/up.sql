-- Your SQL goes here

CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    first_name VARCHAR(35) NOT NULL, 
    last_name VARCHAR(35) NOT NULL,
    username VARCHAR(20) NOT NULL UNIQUE,
    password VARCHAR(122) NOT NULL,
    email VARCHAR(128) UNIQUE NOT NULL UNIQUE,
    joined_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX users__email_idx ON users (email);
 