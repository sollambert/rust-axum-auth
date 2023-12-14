-- Add migration script here
CREATE TABLE "users" (
    id INTEGER PRIMARY KEY,
    uuid VARCHAR(36),
    username VARCHAR(24),
    pass VARCHAR(60),
    email VARCHAR(254)
);