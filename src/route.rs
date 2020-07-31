#![allow(non_snake_case)]
use log::{info};
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};

#[get("/")]
pub(crate) async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body("Hello World! <br/><a href='/hello/Potato'>Test Link</a>")
}

#[get("/hello/{name}")]
pub(crate) async fn indexWithName(_req: HttpRequest, name: web::Path<String>) -> impl Responder {
    info!("Received Name: {}", name);
    HttpResponse::Ok()
        .content_type("text/html")
        .body(format!("Hello: <b>{}</b>!", name))
}
