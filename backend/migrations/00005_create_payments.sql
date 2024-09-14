CREATE TABLE payments (
    -- Unique identifier for each payment
    uuid UUID PRIMARY KEY,
    -- Foreign key to the sender in the users table
    from_username VARCHAR(255) REFERENCES users(username) ON DELETE CASCADE,
    -- Foreign key to the recipient in the users table
    to_username VARCHAR(255) REFERENCES users(username) ON DELETE CASCADE,
    -- Date of the payment
    date TIMESTAMP NOT NULL,
    -- Date when the payment was confirmed (can be NULL)
    confirmation_date TIMESTAMP
);