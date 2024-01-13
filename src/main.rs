#[macro_use]
extern crate rocket;

mod handlers;
mod models;

use handlers::*;

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
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
}
