#![allow(non_snake_case)]
#![feature(proc_macro_hygiene, decl_macro)]

mod route;
use log::{info};
use pretty_env_logger;

#[macro_use] extern crate rocket;
use rocket::config::{Config, Environment};

fn main() {
    pretty_env_logger::init();
    rocket::ignite()
        .mount("/", routes![route::index, route::hello])
        .launch();
}
