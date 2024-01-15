#[macro_use]
extern crate rocket;

mod cors;
mod handlers;
mod models;

use cors::*;
use handlers::*;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![
                create_question,
                read_questions,
                delete_question,
                create_answer,
                read_answers,
                delete_answer
            ],
        )
        .attach(CORS)
}
