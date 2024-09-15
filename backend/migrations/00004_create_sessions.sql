CREATE TABLE sessions (
    uuid UUID PRIMARY KEY,
    expiry TIMESTAMP NOT NULL,
    owner_username VARCHAR(255) REFERENCES users(username) ON DELETE CASCADE
);