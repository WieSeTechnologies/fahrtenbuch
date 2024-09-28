CREATE TABLE payments (
    uuid UUID PRIMARY KEY,
    amount FLOAT NOT NULL,
    from_username VARCHAR(255) REFERENCES users(username) ON DELETE CASCADE,
    to_username VARCHAR(255) REFERENCES users(username) ON DELETE CASCADE,
    creation_date TIMESTAMP NOT NULL,
    confirmation_date TIMESTAMP
);