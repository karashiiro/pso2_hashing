-- Your SQL goes here
CREATE TABLE hash_mapping (
    md5 BYTEA PRIMARY KEY NOT NULL,
    filename TEXT NOT NULL
);