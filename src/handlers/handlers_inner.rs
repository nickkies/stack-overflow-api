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

pub async fn get_questions(
    question_dao: &Box<dyn QuestionDao + Sync + Send>,
) -> Result<Vec<QuestionDetail>, HandlerError> {
    let questions = question_dao.get_questions().await;

    match questions {
        Ok(questions) => Ok(questions),
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
        get_questions_response: Mutex<Option<Result<Vec<QuestionDetail>, DBError>>>,
        delete_question_response: Mutex<Option<Result<(), DBError>>>,
    }

    impl QuestionDaoMock {
        fn new() -> Self {
            Self {
                create_question_response: Mutex::new(None),
                get_questions_response: Mutex::new(None),
                delete_question_response: Mutex::new(None),
            }
        }

        fn mock_create_question(&mut self, response: Result<QuestionDetail, DBError>) {
            self.create_question_response = Mutex::new(Some(response));
        }

        fn mock_get_questions(&mut self, response: Result<Vec<QuestionDetail>, DBError>) {
            self.get_questions_response = Mutex::new(Some(response));
        }

        fn mock_delete_question(&mut self, response: Result<(), DBError>) {
            self.delete_question_response = Mutex::new(Some(response));
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

        async fn get_questions(&self) -> Result<Vec<QuestionDetail>, DBError> {
            self.get_questions_response
                .lock()
                .await
                .take()
                .expect("get questions response should not be None")
        }

        async fn delete_question(&self, _: String) -> Result<(), DBError> {
            self.delete_question_response
                .lock()
                .await
                .take()
                .expect("delete question response should not be None")
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

    #[tokio::test]
    async fn create_question_should_return_question() {
        let question = Question {
            title: "test title".to_string(),
            description: "test description".to_string(),
        };
        let question_detail = QuestionDetail {
            question_uuid: "123".to_string(),
            title: question.title.clone(),
            description: question.description.clone(),
            created_at: "now".to_string(),
        };
        let mut mock_dao = QuestionDaoMock::new();

        mock_dao.mock_create_question(Ok(question_detail.clone()));

        let dao: Box<dyn QuestionDao + Send + Sync> = Box::new(mock_dao);
        let result = create_question(question, &dao).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), question_detail);
    }

    #[tokio::test]
    async fn get_questions_should_return_error() {
        let mut mock_dao = QuestionDaoMock::new();

        mock_dao.mock_get_questions(Err(DBError::InvalidUUID("test".to_string())));

        let dao: Box<dyn QuestionDao + Send + Sync> = Box::new(mock_dao);
        let result = get_questions(&dao).await;

        assert!(result.is_err());
        assert_eq!(
            std::mem::discriminant(&result.unwrap_err()),
            std::mem::discriminant(&HandlerError::InernalError("".to_string()))
        );
    }

    #[tokio::test]
    async fn get_questions_should_return_questions() {
        let question_detail = QuestionDetail {
            question_uuid: "123".to_string(),
            title: "test title".to_string(),
            description: "test description".to_string(),
            created_at: "now".to_string(),
        };
        let mut mock_dao = QuestionDaoMock::new();

        mock_dao.mock_get_questions(Ok(vec![question_detail.clone()]));

        let dao: Box<dyn QuestionDao + Send + Sync> = Box::new(mock_dao);
        let result = get_questions(&dao).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![question_detail]);
    }
}
