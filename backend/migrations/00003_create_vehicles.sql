-- Vehicles Table
CREATE TABLE vehicles (
    -- UUID as the primary key
    uuid UUID PRIMARY KEY,
    -- Name of the vehicle
    name VARCHAR(255) NOT NULL,
    -- Nullable brand field (since it's an Option in Rust)
    brand VARCHAR(255),
    -- Fuel type using an ENUM
    fuel_type fuel_type NOT NULL,
    -- Gasoline type if applicable (nullable if the fuel type is not gasoline)
    gasoline_type gasoline_type,
    -- Foreign key to the users table
    owner_username VARCHAR(255) REFERENCES users(username) ON DELETE CASCADE,
    -- Fuel consumption per 100 km
    fuel_consumption_per_100_km FLOAT NOT NULL,
    -- Fuel consumption per 100 km on the autobahn
    fuel_consumption_per_100_km_autobahn FLOAT NOT NULL,
    -- Maintenance cost per 100 km
    maintenance_cost_per_100_km FLOAT NOT NULL
);