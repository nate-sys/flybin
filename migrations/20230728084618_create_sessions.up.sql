-- Add up migration script here
CREATE TABLE sessions (
    token TEXT PRIMARY KEY,
    id INTEGER NOT NULL,
    FOREIGN KEY(id) REFERENCES admins(id)
);
