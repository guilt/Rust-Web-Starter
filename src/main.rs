mod route;

use rocket::{launch, routes};

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![route::index, route::hello])
}
