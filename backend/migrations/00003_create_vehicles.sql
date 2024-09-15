CREATE TABLE vehicles (
    uuid UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    brand VARCHAR(255),
    fuel_type fuel_type NOT NULL,
    gasoline_type gasoline_type,
    owner_username VARCHAR(255) REFERENCES users(username) ON DELETE CASCADE,
    fuel_consumption_per_100_km FLOAT NOT NULL,
    fuel_consumption_per_100_km_autobahn FLOAT NOT NULL,
    maintenance_cost_per_100_km FLOAT NOT NULL
);