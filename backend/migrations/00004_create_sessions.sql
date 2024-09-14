CREATE TABLE sessions (
    -- UUID for the session as the primary key
    uuid UUID PRIMARY KEY,
    -- Expiry timestamp for the session
    expiry TIMESTAMP NOT NULL,
    -- Foreign key to the users table
    owner_username VARCHAR(255) REFERENCES users(username) ON DELETE CASCADE
);