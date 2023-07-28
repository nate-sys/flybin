-- Add up migration script here
CREATE TABLE admins (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL,
    password CHAR(98) NOT NULL  -- argon2id
);
