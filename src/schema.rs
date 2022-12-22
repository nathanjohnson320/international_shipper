// @generated automatically by Diesel CLI.

diesel::table! {
    country_codes (id) {
        id -> Uuid,
        name -> Varchar,
        short_code -> Varchar,
        long_code -> Varchar,
        numeric_code -> Varchar,
    }
}

diesel::table! {
    hts_codes (id) {
        id -> Uuid,
        hts_number -> Varchar,
        description -> Text,
        unit_of_quantity -> Nullable<Varchar>,
        general_rate_of_duty -> Nullable<Text>,
        special_rate_of_duty -> Nullable<Text>,
        column_2_rate_of_duty -> Nullable<Text>,
        additional_duties -> Nullable<Text>,
        hts_code_id -> Nullable<Uuid>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    country_codes,
    hts_codes,
);
