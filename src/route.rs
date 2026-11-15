use rocket::get;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world! Hit /hello/Potato"
}

#[get("/hello/<name>")]
pub fn hello(name: String) -> String {
    format!("Hello {name}!")
}
