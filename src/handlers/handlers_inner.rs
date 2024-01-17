use crate::persistance::question_dao::QuestionDao;

use super::{Question, QuestionDetail};

#[derive(Debug, PartialEq)]
pub enum HandlerError {
    BadRequest(String),
    InernalError(String),
}

impl HandlerError {
    pub fn default_internal_error() -> Self {
        HandlerError::InernalError("Something went wrong! Please try again.".to_string())
    }
}

pub async fn create_question(
    question: Question,
    question_dao: &Box<dyn QuestionDao + Sync + Send>,
) -> Result<QuestionDetail, HandlerError> {
    let question = question_dao.create_question(question).await;

    match question {
        Ok(question) => Ok(question),
        Err(e) => {
            error!("{e:?}");
            Err(HandlerError::default_internal_error())
        }
    }
}
