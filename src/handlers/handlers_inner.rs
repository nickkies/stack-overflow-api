use crate::{
    models::{DBError, Question, QuestionDetail},
    persistance::question_dao::QuestionDao,
};

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

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::Mutex;

    struct QuestionDaoMock {
        create_question_response: Mutex<Option<Result<QuestionDetail, DBError>>>,
    }

    impl QuestionDaoMock {
        fn new() -> Self {
            Self {
                create_question_response: Mutex::new(None),
            }
        }

        fn mock_create_question(&mut self, response: Result<QuestionDetail, DBError>) {
            self.create_question_response = Mutex::new(Some(response))
        }
    }

    #[async_trait]
    impl QuestionDao for QuestionDaoMock {
        async fn create_question(&self, _: Question) -> Result<QuestionDetail, DBError> {
            self.create_question_response
                .lock()
                .await
                .take()
                .expect("create question response should not be None.")
        }
    }
}
