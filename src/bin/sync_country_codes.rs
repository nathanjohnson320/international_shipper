use diesel::prelude::*;
use headless_chrome::Browser;
use international_shipper::connection_pool;
use international_shipper::models::{CountryCode, NewCountryCode};
use international_shipper::schema::country_codes;
use std::{thread, time};

pub fn main() {
    let browser = Browser::default().expect("Could not start browser");

    let tab = browser
        .wait_for_initial_tab()
        .expect("Could not open new tab");

    let radios = tab
        .navigate_to("https://www.iso.org/obp/ui/#search")
        .expect("Could not navigate to ISO country code search")
        .wait_for_elements("input[type=radio]")
        .expect("Could not find country codes radio button");

    radios
        .last()
        .expect("Could not find last radio button")
        .click()
        .expect("Could not click last radio button");

    tab.wait_for_element(".v-slot-go")
        .expect("Could not find submit button")
        .click()
        .expect("Could not click submit");

    tab.wait_for_element("select")
        .expect("Could not find page size select")
        .type_into("300")
        .expect("Could not set page limit")
        .click()
        .expect("Could not confirm page limit");

    // There's no negative test so we need to wait a few seconds
    // alternatively run through the pages and not do this
    let wait_time = time::Duration::from_millis(3000);
    thread::sleep(wait_time);

    let rows = tab
        .wait_for_elements(".v-grid-row")
        .expect("Could not find table rows");

    let conn = &mut connection_pool().get().unwrap();
    for row in rows.iter().skip(1) {
        let cells = row
            .find_elements(".v-grid-cell")
            .expect("Could not find table cell");

        let name = cells[0].get_inner_text().expect("Could not get cell name");
        let short_code = cells[2].get_inner_text().expect("Could not get short code");
        let long_code = cells[3].get_inner_text().expect("Could not get long code");
        let numeric_code = cells[4]
            .get_inner_text()
            .expect("Could not get numeric code");

        let code: CountryCode = diesel::insert_into(country_codes::table)
            .values(NewCountryCode {
                name: &name,
                short_code: &short_code,
                long_code: &long_code,
                numeric_code: &numeric_code,
            })
            .get_result(conn)
            .expect("Error saving country code");
        println!("Saved code {}: {}", code.id, code.name);
    }
}
