-- Your SQL goes here
CREATE TABLE hash_mapping (
    md5 UUID PRIMARY KEY NOT NULL,
    filename TEXT NOT NULL
);