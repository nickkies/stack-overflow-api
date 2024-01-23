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
        let uuid = sqlx::types::Uuid::parse_str(&answer.question_uuid).map_err(|_| {
            DBError::InvalidUUID(format!(
                "Could not parse answer UUID: {}",
                answer.question_uuid
            ))
        })?;

        Ok(AnswerDetail {
            answer_uuid: "123".to_string(),
            question_uuid: "123".to_string(),
            content: "test content".to_string(),
            created_at: "now".to_string(),
        })
    }

    async fn get_answers(&self, question_uuid: String) -> Result<Vec<AnswerDetail>, DBError> {
        todo!();
    }

    async fn delete_answer(&self, answer_uuid: String) -> Result<(), DBError> {
        todo!();
    }
}
