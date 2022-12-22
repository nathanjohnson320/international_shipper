use diesel::prelude::*;
use international_shipper::{
    connection_pool,
    models::{HTSCode, NewHTSCode},
    schema::hts_codes,
};
use reqwest::multipart::Form;

#[tokio::main]
async fn main() {
    let form = Form::new()
        .text("format", "csv")
        .text("from", "0000.00.00.00")
        .text("to", "9999.99.99.99")
        .text("styles", "true")
        .text("Submit", "Submit");

    let client = reqwest::Client::new();
    let data = client
        .post("https://hts.usitc.gov/api/export")
        .multipart(form)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let mut rdr = csv::Reader::from_reader(data.as_bytes());
    let mut indent = -1;
    let mut parent: Option<uuid::Uuid> = None;

    let conn = &mut connection_pool().get().unwrap();

    for result in rdr.records() {
        let record = result.unwrap();

        let hts_number = record.get(0).unwrap().parse::<String>().unwrap();
        let next_indent = record.get(1).unwrap().parse::<i32>().unwrap();
        let description = record.get(2).unwrap().parse::<String>().unwrap();
        let unit_of_quantity = record.get(3).unwrap().parse::<String>().unwrap();
        let general_rate_of_duty = record.get(4).unwrap().parse::<String>().unwrap();
        let special_rate_of_duty = record.get(5).unwrap().parse::<String>().unwrap();
        let column_2_rate_of_duty = record.get(6).unwrap().parse::<String>().unwrap();
        let additional_duties = record.get(8).unwrap().parse::<String>().unwrap();

        let mut new_code = NewHTSCode {
            hts_number: &hts_number,
            description: &description,
            unit_of_quantity: Some(unit_of_quantity),
            general_rate_of_duty: Some(general_rate_of_duty),
            special_rate_of_duty: Some(special_rate_of_duty),
            column_2_rate_of_duty: Some(column_2_rate_of_duty),
            additional_duties: Some(additional_duties),
            hts_code_id: None,
        };

        if indent == next_indent {
            new_code.hts_code_id = Some(parent.unwrap());
            let code: HTSCode = diesel::insert_into(hts_codes::table)
                .values(new_code)
                .get_result(conn)
                .expect("Error saving hts code");
            println!(
                "Created code {} with parent {}",
                code.hts_number,
                parent.as_mut().unwrap()
            );
        } else {
            match parent {
                Some(id) => new_code.hts_code_id = Some(id),
                None => (),
            }

            let code: HTSCode = diesel::insert_into(hts_codes::table)
                .values(new_code)
                .get_result(conn)
                .expect("Error saving hts code");
            parent = Some(code.id);

            println!("Created code {}", code.hts_number);
        }
        indent = next_indent;
    }
}
