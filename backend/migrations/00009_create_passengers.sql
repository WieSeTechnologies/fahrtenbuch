CREATE TYPE passenger_payment_adjustment_type AS ENUM ('none', 'tip', 'roundupto');
CREATE TABLE passengers (
    id SERIAL PRIMARY KEY,
    trip_id INTEGER NOT NULL REFERENCES trips(id) ON DELETE CASCADE,
    username VARCHAR(255) NOT NULL REFERENCES users(username) ON DELETE CASCADE,
    payment_adjustment passenger_payment_adjustment_type DEFAULT 'none',
    payment_adjustment_amount FLOAT,
    distance_traveled_percent FLOAT NOT NULL,
    is_paying BOOLEAN NOT NULL
);