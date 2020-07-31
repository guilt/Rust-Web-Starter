#![allow(non_snake_case)]
mod route;
use log::{info};
use pretty_env_logger;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();
    let address = "0.0.0.0:8080";
    info!("Starting Web Server: {}", address);
    HttpServer::new(|| {
        App::new()
            .service(route::index)
            .service(route::indexWithName)
    })
    .bind(address)?
    .run()
    .await
}
