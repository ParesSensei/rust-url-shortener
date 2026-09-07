-- Add migration script here
CREATE TABLE urls (
                      id BIGSERIAL PRIMARY KEY,
                      short_code VARCHAR(8) NOT NULL UNIQUE,
                      original_url TEXT NOT NULL
);