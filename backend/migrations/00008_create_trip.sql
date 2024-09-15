CREATE TYPE trip_type AS ENUM ('template', 'custom');
CREATE TABLE trips (
    id SERIAL PRIMARY KEY,
    trip_type trip_type NOT NULL,
    trip_template VARCHAR(255),
    trip_route trip_route,
    vehicle UUID REFERENCES vehicles(uuid) ON DELETE CASCADE
);