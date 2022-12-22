use crate::AppData;
use diesel::prelude::*;
use international_shipper::models::{CountryCode, NewCountryCode, UpdateCountryCode};
use international_shipper::schema::country_codes;
use international_shipper::schema::country_codes::dsl::*;
use rocket::serde::json::Json;
use rocket::State;

#[get("/")]
pub fn index(app_state: &State<AppData>) -> Json<Vec<CountryCode>> {
    let results = country_codes
        .load::<CountryCode>(&mut app_state.db.get().unwrap())
        .expect("Error loading posts");

    Json(results)
}

#[post("/", data = "<new_country_code>")]
pub fn create(
    new_country_code: Json<NewCountryCode>,
    app_state: &State<AppData>,
) -> Json<CountryCode> {
    let country_code: CountryCode = diesel::insert_into(country_codes::table)
        .values(&new_country_code.into_inner())
        .get_result(&mut app_state.db.get().unwrap())
        .expect("Error saving new country code");

    Json(country_code)
}

#[put("/", data = "<update_country_code>")]
pub fn update(
    update_country_code: Json<UpdateCountryCode>,
    app_state: &State<AppData>,
) -> Json<CountryCode> {
    let updated_country_code = diesel::update(country_codes::table)
        .set(&update_country_code.into_inner())
        .get_result(&mut app_state.db.get().unwrap())
        .expect("Error updating country code");

    Json(updated_country_code)
}

#[patch("/<country_code_id>", data = "<update_country_code>")]
pub fn patch(
    country_code_id: &str,
    update_country_code: Json<UpdateCountryCode>,
    app_state: &State<AppData>,
) -> Json<CountryCode> {
    let uuid = uuid::Uuid::parse_str(country_code_id).unwrap();
    let updated_country_code = diesel::update(country_codes.filter(id.eq(uuid)))
        .set(&update_country_code.into_inner())
        .get_result(&mut app_state.db.get().unwrap())
        .expect("Error updating country code");

    Json(updated_country_code)
}
