-- Add up migration script here
CREATE TABLE pastes (
    slug varchar(6) PRIMARY KEY NOT NULL,
    content text NOT NULL,
    created_at timestamp NOT NULL,
    expires_at timestamp NOT NULL,
    secret varchar(32) NOT NULL,
    ip_address text NOT NULL,
    password blob
)
