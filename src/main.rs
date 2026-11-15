mod route;

#[cfg(target_family = "rust9x")]
mod win9x;

use rocket::{launch, routes};

#[launch]
fn rocket() -> _ {
    #[cfg(target_family = "rust9x")]
    win9x::keep_shims();

    rocket::build().mount("/", routes![route::index, route::hello])
}
