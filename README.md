international_shipper
=====================
API that pulls data from HS, HTS apps and serves it via REST

## Prerequisites

* Make sure you have the postgres libpq (sudo apt install libpq-dev)
* add a .env file with basic db credentials `DATABASE_URL=postgres://postgres:postgres@localhost:5432/international_shipper_dev`
* Install the diesel CLI to run migrations `cargo install diesel_cli --no-default-features --features postgres`
    * if you're using asdf you might need to `asdf reshim rust` to add it to the path
    * `diesel setup`
    * `diesel migration run`

## Data Sources

* https://www.iso.org/obp/ui/#search
* https://hts.usitc.gov/export
* https://www.census.gov/foreign-trade/schedules/b/2022/exp-code.txt
* https://www.census.gov/foreign-trade/schedules/b/2022/exp-stru.txt