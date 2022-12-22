use crate::schema::country_codes;
use crate::schema::hts_codes;
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct CountryCode {
    pub id: uuid::Uuid,
    pub name: String,
    pub short_code: String,
    pub long_code: String,
    pub numeric_code: String,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = country_codes)]
pub struct NewCountryCode<'a> {
    pub name: &'a str,
    pub short_code: &'a str,
    pub long_code: &'a str,
    pub numeric_code: &'a str,
}

#[derive(Deserialize, AsChangeset)]
#[diesel(table_name = country_codes)]
pub struct UpdateCountryCode<'a> {
    pub name: Option<&'a str>,
    pub short_code: Option<&'a str>,
    pub long_code: Option<&'a str>,
    pub numeric_code: Option<&'a str>,
}

#[derive(Serialize, Queryable)]
#[serde(crate = "rocket::serde")]
#[diesel(belongs_to(HTSCode))]
pub struct HTSCode {
    pub id: uuid::Uuid,
    pub hts_number: String,
    pub description: String,
    pub unit_of_quantity: Option<String>,
    pub general_rate_of_duty: Option<String>,
    pub special_rate_of_duty: Option<String>,
    pub column_2_rate_of_duty: Option<String>,
    pub additional_duties: Option<String>,
    pub hts_code_id: Option<uuid::Uuid>,
}

#[derive(Serialize, Deserialize, Insertable)]
#[diesel(table_name = hts_codes)]

pub struct NewHTSCode<'a> {
    pub hts_number: &'a str,
    pub description: &'a str,
    pub unit_of_quantity: Option<String>,
    pub general_rate_of_duty: Option<String>,
    pub special_rate_of_duty: Option<String>,
    pub column_2_rate_of_duty: Option<String>,
    pub additional_duties: Option<String>,
    pub hts_code_id: Option<uuid::Uuid>,
}
