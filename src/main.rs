#[macro_use]
extern crate rocket;

mod models;

#[get("/hi")]
fn init() -> String {
    format!("Hi there!")
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api/v1", routes![init])
}
