-- Add migration script here
CREATE TABLE "users" (
    id INTEGER PRIMARY KEY,
    username VARCHAR(24),
    pass VARCHAR(24)
);