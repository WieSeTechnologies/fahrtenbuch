CREATE TYPE user_role AS ENUM ('Restricted', 'Normal', 'Admin');
CREATE TABLE users (
    -- Username is the primary key
    username VARCHAR(255) PRIMARY KEY,
    -- Display name for the user
    displayname VARCHAR(255) NOT NULL,
    -- The password hash
    password_hash VARCHAR(255) NOT NULL,
    -- Indicates if the user needs to reset their password
    new_password_required BOOLEAN DEFAULT FALSE,
    -- User role with default value as 'Normal'
    role user_role NOT NULL DEFAULT 'Normal'
);