#![feature(plugin, custom_derive, custom_attribute, const_fn)]
#![plugin(rocket_codegen)]

extern crate chrono;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_infer_schema;
extern crate dotenv;
#[macro_use]
extern crate dotenv_codegen;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

pub mod app;
pub mod schema;
pub mod database;
pub mod models;

use dotenv::dotenv;

fn main() {
    use app::{errors, index, todo};

    dotenv().ok();

    rocket::ignite()
        .manage(database::init_pool())
        .mount("/", routes![index::index, index::hi])
        .mount("todos", routes![todo::new])
        .catch(errors![
            errors::bad_request,
            errors::not_found,
            errors::internal_error
        ])
        .launch();
}
