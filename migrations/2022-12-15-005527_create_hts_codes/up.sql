CREATE TABLE hts_codes (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4() NOT NULL,
    hts_number VARCHAR(15) NOT NULL,
    description TEXT NOT NULL,
    unit_of_quantity VARCHAR(255),
    general_rate_of_duty TEXT,
    special_rate_of_duty TEXT,
    column_2_rate_of_duty TEXT,
    additional_duties TEXT,
    hts_code_id uuid,
    CONSTRAINT fk_parent FOREIGN KEY(hts_code_id) REFERENCES hts_codes(id)
);
CREATE INDEX hts_codes_hts_number_idx ON hts_codes (hts_number);
CREATE INDEX hts_codes_description_idx ON hts_codes (description);