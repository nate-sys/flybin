-- Add up migration script here
CREATE TABLE admins (
    id AUTO_INCREMENT PRIMARY KEY,
    username TEXT NOT NULL,
    password CHAR(98) NOT NULL  -- argon2id
);
