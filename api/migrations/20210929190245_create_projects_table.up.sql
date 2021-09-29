-- Add up migration script here

CREATE TABLE projects (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    created_at DATE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
