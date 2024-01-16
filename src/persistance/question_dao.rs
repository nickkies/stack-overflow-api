use sqlx::PgPool;

use crate::models::{DBError, Question, QuestionDetail, QuestionId};

#[async_trait]
pub trait QuestionDao {
    async fn create_question(&self, question: Question) -> Result<QuestionDetail, DBError>;
    async fn get_questions(&self) -> Result<Vec<QuestionDetail>, DBError>;
    async fn delete_question(&self, question_uuid: QuestionId) -> Result<(), DBError>;
}

pub struct QuestionDaoImpl {
    db: PgPool,
}

impl QuestionDaoImpl {
    pub fn new(db: PgPool) -> Self {
        QuestionDaoImpl { db }
    }
}

#[async_trait]
impl QuestionDao for QuestionDaoImpl {
    async fn create_question(&self, qustion: Question) -> Result<QuestionDetail, DBError> {
        todo!();
    }

    async fn get_questions(&self) -> Result<Vec<QuestionDetail>, DBError> {
        todo!();
    }

    async fn delete_question(&self, question_uuid: QuestionId) -> Result<(), DBError> {
        todo!();
    }
}
