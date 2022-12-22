use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use international_shipper::connection_pool;

mod v1;

#[macro_use]
extern crate rocket;

pub struct AppData {
    db: Pool<ConnectionManager<PgConnection>>,
}

#[launch]
fn rocket() -> _ {
    let connection_pool: Pool<ConnectionManager<PgConnection>> = connection_pool();

    rocket::build()
        .manage(AppData {
            db: connection_pool,
        })
        .mount("/v1", routes![v1::index])
        .mount(
            "/v1/country_codes",
            routes![
                v1::country_codes::index,
                v1::country_codes::create,
                v1::country_codes::update,
                v1::country_codes::patch
            ],
        )
}
