#![allow(non_snake_case)]
#![feature(proc_macro_hygiene, decl_macro)]

mod route;
use pretty_env_logger;

#[macro_use] extern crate rocket;

fn main() {
    pretty_env_logger::init();
    rocket::ignite()
        .mount("/", routes![route::index, route::hello])
        .launch();
}
