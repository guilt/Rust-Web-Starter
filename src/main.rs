#![allow(non_snake_case)]

mod route;
use pretty_env_logger;

#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _ {
    pretty_env_logger::init();
    rocket::build()
        .mount("/", routes![route::index, route::hello])
}
