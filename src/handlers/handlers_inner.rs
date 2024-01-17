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

    #[tokio::test]
    async fn create_question_should_return_error() {
        let question = Question {
            title: "test title".to_string(),
            description: "test description".to_string(),
        };
        let mut mock_dao = QuestionDaoMock::new();

        mock_dao.mock_create_question(Err(DBError::InvalidUUID("test".to_string())));

        let dao: Box<dyn QuestionDao + Send + Sync> = Box::new(mock_dao);
        let result = create_question(question, &dao).await;

        assert!(result.is_err());
        assert_eq!(
            std::mem::discriminant(&result.unwrap_err()),
            std::mem::discriminant(&HandlerError::InernalError("".to_string()))
        );
    }
}
