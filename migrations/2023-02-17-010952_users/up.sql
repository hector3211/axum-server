-- Your SQL goes here

CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  username varchar NOT NULL,
  hashed_password varchar NOT NULL
);

