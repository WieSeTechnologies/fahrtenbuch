CREATE TYPE fuel_type AS ENUM ('gasoline', 'diesel', 'electric', 'other');
CREATE TYPE gasoline_type AS ENUM ('super', 'supere10', 'superplus');
CREATE TABLE fuel_prices (
    id SERIAL PRIMARY KEY,
    price FLOAT NOT NULL,
    date DATE NOT NULL,
    fuel_type fuel_type NOT NULL,
    gasoline_type gasoline_type
);