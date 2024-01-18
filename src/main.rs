#[macro_use]
extern crate rocket;

extern crate log;

extern crate pretty_env_logger;

mod cors;
mod handlers;
mod models;
mod persistance;

use cors::*;
use dotenvy::dotenv;
use handlers::*;
use persistance::question_dao::{QuestionDao, QuestionDaoImpl};
use sqlx::postgres::PgPoolOptions;

#[launch]
async fn rocket() -> _ {
    pretty_env_logger::init();
    dotenv().ok();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL must be set."))
        .await
        .expect("Failed to create Postgres connection pool!");

    let question_dao = QuestionDaoImpl::new(pool);

    rocket::build()
        .mount(
            "/",
            routes![
                create_question,
                get_questions,
                delete_question,
                create_answer,
                get_answers,
                delete_answer
            ],
        )
        .attach(CORS)
        .manage(Box::new(question_dao) as Box<dyn QuestionDao + Send + Sync>)
}
