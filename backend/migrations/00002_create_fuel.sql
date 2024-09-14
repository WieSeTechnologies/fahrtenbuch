-- Enum for FuelType
CREATE TYPE fuel_type AS ENUM ('gasoline', 'diesel', 'electric', 'other');
-- Enum for GasolineType
CREATE TYPE gasoline_type AS ENUM ('super', 'supere10', 'superplus');
-- Fuel Prices Table
CREATE TABLE fuel_prices (
    -- Auto-incremented primary key for fuel prices
    id SERIAL PRIMARY KEY,
    -- Price of fuel
    price FLOAT NOT NULL,
    -- Date of the fuel price (converted from the Rust String to a SQL DATE type)
    date DATE NOT NULL
);