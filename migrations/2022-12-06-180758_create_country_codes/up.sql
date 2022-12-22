CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE country_codes (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4() NOT NULL,
    name VARCHAR(255) NOT NULL,
    short_code VARCHAR(2) NOT NULL,
    long_code VARCHAR(3) NOT NULL,
    numeric_code VARCHAR(3) NOT NULL
);
CREATE INDEX country_codes_name_idx ON country_codes (name);
CREATE INDEX country_codes_short_code_idx ON country_codes (short_code);
CREATE INDEX country_codes_long_code_idx ON country_codes (long_code);
CREATE INDEX country_codes_numeric_code_idx ON country_codes (numeric_code);