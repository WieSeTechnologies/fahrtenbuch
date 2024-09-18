CREATE TYPE user_role AS ENUM ('restricted', 'normal', 'admin');
CREATE TABLE users (
    username VARCHAR(255) PRIMARY KEY,
    displayname VARCHAR(255) NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    new_password_required BOOLEAN DEFAULT FALSE,
    creation_date TIMESTAMPTZ NOT NULL,
    role user_role NOT NULL DEFAULT 'normal'
);