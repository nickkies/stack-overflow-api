use crate::{
    models::{Answer, AnswerDetail, AnswerId, DBError, Question, QuestionDetail, QuestionId},
    persistance::{answer_dao::AnswerDao, question_dao::QuestionDao},
};

#[derive(Debug, PartialEq)]
pub enum HandlerError {
    BadRequest(String),
    InternalError(String),
}

impl HandlerError {
    pub fn default_internal_error() -> Self {
        HandlerError::InternalError("Something went wrong! Please try again.".to_string())
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

pub async fn delete_question(
    question_id: QuestionId,
    question_dao: &Box<dyn QuestionDao + Send + Sync>,
) -> Result<(), HandlerError> {
    let result = question_dao
        .delete_question(question_id.question_uuid)
        .await;

    match result {
        Ok(()) => Ok(()),
        Err(e) => {
            error!("{e:?}");
            Err(HandlerError::default_internal_error())
        }
    }
}

pub async fn create_answer(
    answer: Answer,
    answer_dao: &Box<dyn AnswerDao + Send + Sync>,
) -> Result<AnswerDetail, HandlerError> {
    let answer = answer_dao.create_answer(answer).await;

    match answer {
        Ok(answer) => Ok(answer),
        Err(e) => {
            error!("{e:?}");

            match e {
                DBError::InvalidUUID(s) => Err(HandlerError::BadRequest(s)),
                _ => Err(HandlerError::default_internal_error()),
            }
        }
    }
}

pub async fn get_answers(
    question_id: QuestionId,
    answer_dao: &Box<dyn AnswerDao + Send + Sync>,
) -> Result<Vec<AnswerDetail>, HandlerError> {
    todo!();
}

#[cfg(test)]
mod tests {
    use crate::models::QuestionId;

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

    struct AnswerDaoMock {
        create_answer_response: Mutex<Option<Result<AnswerDetail, DBError>>>,
        get_answers_response: Mutex<Option<Result<Vec<AnswerDetail>, DBError>>>,
        delete_answer_response: Mutex<Option<Result<(), DBError>>>,
    }

    impl AnswerDaoMock {
        fn new() -> Self {
            Self {
                create_answer_response: Mutex::new(None),
                get_answers_response: Mutex::new(None),
                delete_answer_response: Mutex::new(None),
            }
        }

        pub fn mock_create_answer(&mut self, response: Result<AnswerDetail, DBError>) {
            self.create_answer_response = Mutex::new(Some(response));
        }

        pub fn mock_get_answers(&mut self, response: Result<Vec<AnswerDetail>, DBError>) {
            self.get_answers_response = Mutex::new(Some(response));
        }

        pub fn mock_delete_answer(&mut self, response: Result<(), DBError>) {
            self.delete_answer_response = Mutex::new(Some(response));
        }
    }

    #[async_trait]
    impl AnswerDao for AnswerDaoMock {
        async fn create_answer(&self, _: Answer) -> Result<AnswerDetail, DBError> {
            self.create_answer_response
                .lock()
                .await
                .take()
                .expect("create answer response should not be None")
        }

        async fn get_answers(&self, _: String) -> Result<Vec<AnswerDetail>, DBError> {
            self.get_answers_response
                .lock()
                .await
                .take()
                .expect("get answers response should not be None")
        }

        async fn delete_answer(&self, _: String) -> Result<(), DBError> {
            self.delete_answer_response
                .lock()
                .await
                .take()
                .expect("delete answer response should not be None")
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
            std::mem::discriminant(&HandlerError::InternalError("".to_string()))
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
            std::mem::discriminant(&HandlerError::InternalError("".to_string()))
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

    #[tokio::test]
    async fn delete_question_should_return_error() {
        let question_id = QuestionId {
            question_uuid: "123".to_string(),
        };
        let mut mock_dao = QuestionDaoMock::new();

        mock_dao.mock_delete_question(Err(DBError::InvalidUUID("test".to_string())));

        let dao: Box<dyn QuestionDao + Send + Sync> = Box::new(mock_dao);
        let result = delete_question(question_id, &dao).await;

        assert!(result.is_err());
        assert_eq!(
            std::mem::discriminant(&result.unwrap_err()),
            std::mem::discriminant(&HandlerError::InternalError("".to_string()))
        );
    }

    #[tokio::test]
    async fn delete_question_should_succeed() {
        let question_id = QuestionId {
            question_uuid: "123".to_string(),
        };
        let mut mock_dao = QuestionDaoMock::new();

        mock_dao.mock_delete_question(Ok(()));

        let dao: Box<dyn QuestionDao + Send + Sync> = Box::new(mock_dao);
        let result = delete_question(question_id, &dao).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ());
    }

    #[tokio::test]
    async fn create_answer_should_return_bad_request_error() {
        let answer = Answer {
            question_uuid: "123".to_string(),
            content: "test content".to_string(),
        };
        let mut mock_dao = AnswerDaoMock::new();

        mock_dao.mock_create_answer(Err(DBError::InvalidUUID("test".to_string())));

        let dao: Box<dyn AnswerDao + Send + Sync> = Box::new(mock_dao);
        let result = create_answer(answer, &dao).await;

        assert!(result.is_err());
        assert_eq!(
            std::mem::discriminant(&result.unwrap_err()),
            std::mem::discriminant(&HandlerError::BadRequest("".to_string()))
        );
    }

    #[tokio::test]
    async fn create_answer_should_return_internal_error() {
        let answer = Answer {
            question_uuid: "123".to_string(),
            content: "test content".to_string(),
        };
        let mut mock_dao = AnswerDaoMock::new();

        mock_dao.mock_create_answer(Err(DBError::Other(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "oh no!",
        )))));

        let dao: Box<dyn AnswerDao + Send + Sync> = Box::new(mock_dao);
        let result = create_answer(answer, &dao).await;

        assert!(result.is_err());
        assert_eq!(
            std::mem::discriminant(&result.unwrap_err()),
            std::mem::discriminant(&HandlerError::InternalError("".to_string()))
        );
    }

    #[tokio::test]
    async fn create_answer_should_return_answer() {
        let answer = Answer {
            question_uuid: "123".to_string(),
            content: "test content".to_string(),
        };
        let answer_detail = AnswerDetail {
            answer_uuid: "456".to_string(),
            question_uuid: answer.question_uuid.clone(),
            content: answer.content.clone(),
            created_at: "now".to_string(),
        };
        let mut mock_dao = AnswerDaoMock::new();

        mock_dao.mock_create_answer(Ok(answer_detail.clone()));

        let dao: Box<dyn AnswerDao + Send + Sync> = Box::new(mock_dao);
        let result = create_answer(answer, &dao).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), answer_detail);
    }

    #[tokio::test]
    async fn get_answers_should_return_error() {
        let question_id = QuestionId {
            question_uuid: "123".to_string(),
        };
        let mut mock_dao = AnswerDaoMock::new();

        mock_dao.mock_get_answers(Err(DBError::InvalidUUID("test".to_string())));

        let dao: Box<dyn AnswerDao + Send + Sync> = Box::new(mock_dao);
        let result = get_answers(question_id, &dao).await;

        assert!(result.is_err());
        assert_eq!(
            std::mem::discriminant(&result.unwrap_err()),
            std::mem::discriminant(&HandlerError::InternalError("".to_string()))
        );
    }

    #[tokio::test]
    async fn get_answers_should_return_answers() {
        let answer_detail = AnswerDetail {
            question_uuid: "123".to_string(),
            answer_uuid: "456".to_string(),
            content: "test content".to_string(),
            created_at: "now".to_string(),
        };
        let question_id = QuestionId {
            question_uuid: "123".to_string(),
        };
        let mut mock_dao = AnswerDaoMock::new();

        mock_dao.mock_get_answers(Ok(vec![answer_detail.clone()]));

        let dao: Box<dyn AnswerDao + Send + Sync> = Box::new(mock_dao);
        let result = get_answers(question_id, &dao).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![answer_detail]);
    }
}
