-- Your SQL goes here
CREATE TABLE todos (
    id SERIAL PRIMARY KEY,
    title varchar NOT NULL,
    body varchar NOT NULL,
    completed BOOLEAN NOT NULL DEFAULT FALSE,
    user_id INT NOT NULL REFERENCES users (id)
);

