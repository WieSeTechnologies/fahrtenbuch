-- CREATE TYPE fuel_type AS ENUM ('gasoline', 'diesel', 'electric', 'other');
-- CREATE TYPE gasoline_type AS ENUM ('super', 'supere10', 'superplus');
CREATE TABLE fuel_prices (
    id SERIAL PRIMARY KEY,
    price FLOAT4 NOT NULL,
    date TIMESTAMPTZ NOT NULL,
    fuel_type INT4 NOT NULL,
    gasoline_type INT4
);