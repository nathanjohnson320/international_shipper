pub mod country_codes;

#[get("/")]
pub fn index() -> &'static str {
    "Ok"
}
