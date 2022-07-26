#![allow(non_snake_case)]
extern crate rocket;

#[get("/")]
pub fn index() -> &'static str {
    "Hello World!. Hit /hello/Potato"
}

#[get("/hello/<name>")]
pub fn hello(name: String) -> String {
    format!("Hello {}!", name)
}
