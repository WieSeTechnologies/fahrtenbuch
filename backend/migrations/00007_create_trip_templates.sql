CREATE TABLE trip_templates (
    id SERIAL PRIMARY KEY,
    from_loc_name VARCHAR(255) NOT NULL,
    to_loc_name VARCHAR(255) NOT NULL,
    trip_route trip_route NOT NULL
);