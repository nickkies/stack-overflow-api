use crate::{
    models::*,
    persistance::{answer_dao::AnswerDao, question_dao::QuestionDao},
};
use rocket::{serde::json::Json, State};

use self::handlers_inner::HandlerError;

mod handlers_inner;

#[derive(Responder)]
pub enum APIError {
    #[response(status = 400)]
    BadRequest(String),
    #[response(status = 500)]
    InternalError(String),
}

impl From<HandlerError> for APIError {
    fn from(value: HandlerError) -> Self {
        match value {
            HandlerError::BadRequest(s) => Self::BadRequest(s),
            HandlerError::InternalError(s) => Self::InternalError(s),
        }
    }
}

#[post("/question", data = "<question>")]
pub async fn create_question(
    question: Json<Question>,
    question_dao: &State<Box<dyn QuestionDao + Sync + Send>>,
) -> Result<Json<QuestionDetail>, APIError> {
    match handlers_inner::create_question(question.0, question_dao.inner()).await {
        Ok(res) => Ok(Json(res)),
        Err(err) => Err(err.into()),
    }
}

#[get("/questions")]
pub async fn get_questions(
    question_dao: &State<Box<dyn QuestionDao + Sync + Send>>,
) -> Result<Json<Vec<QuestionDetail>>, APIError> {
    match handlers_inner::get_questions(question_dao.inner()).await {
        Ok(res) => Ok(Json(res)),
        Err(err) => Err(err.into()),
    }
}

#[delete("/question", data = "<question_uuid>")]
pub async fn delete_question(
    question_uuid: Json<QuestionId>,
    question_dao: &State<Box<dyn QuestionDao + Sync + Send>>,
) -> Result<(), APIError> {
    match handlers_inner::delete_question(question_uuid.0, question_dao.inner()).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e.into()),
    }
}

#[post("/answer", data = "<answer>")]
pub async fn create_answer(
    answer: Json<Answer>,
    answer_dao: &State<Box<dyn AnswerDao + Send + Sync>>,
) -> Result<Json<AnswerDetail>, APIError> {
    match handlers_inner::create_answer(answer.0, answer_dao.inner()).await {
        Ok(res) => Ok(Json(res)),
        Err(err) => Err(err.into()),
    }
}

#[get("/answers", data = "<question_uuid>")]
pub async fn get_answers(
    question_uuid: Json<QuestionId>,
    answer_dao: &State<Box<dyn AnswerDao + Send + Sync>>,
) -> Result<Json<Vec<AnswerDetail>>, APIError> {
    match handlers_inner::get_answers(question_uuid.0, answer_dao.inner()).await {
        Ok(res) => Ok(Json(res)),
        Err(err) => Err(err.into()),
    }
}

#[delete("/answer", data = "<answer_uuid>")]
pub async fn delete_answer(
    answer_uuid: Json<AnswerId>,
    answer_dao: &State<Box<dyn AnswerDao + Send + Sync>>,
) -> Result<(), APIError> {
    match handlers_inner::delete_answer(answer_uuid.0, answer_dao.inner()).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e.into()),
    }
}
