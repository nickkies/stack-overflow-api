use sqlx::PgPool;

use crate::models::{Answer, AnswerDetail, DBError};

#[async_trait]
pub trait AnswerDao {
    async fn create_answer(&self, answer: Answer) -> Result<AnswerDetail, DBError>;
    async fn get_answers(&self, question_uuid: String) -> Result<Vec<AnswerDetail>, DBError>;
    async fn delete_answer(&self, answer_uuid: String) -> Result<(), DBError>;
}

pub struct AnswerDaoImpl {
    db: PgPool,
}

impl AnswerDaoImpl {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
}

#[async_trait]
impl AnswerDao for AnswerDaoImpl {
    async fn create_answer(&self, answer: Answer) -> Result<AnswerDetail, DBError> {
        todo!();
    }

    async fn get_answers(&self, question_uuid: String) -> Result<Vec<AnswerDetail>, DBError> {
        todo!();
    }

    async fn delete_answer(&self, answer_uuid: String) -> Result<(), DBError> {
        todo!();
    }
}
