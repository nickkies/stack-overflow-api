use async_trait::async_trait;
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
    async fn create_question(&self, question: Question) -> Result<QuestionDetail, DBError> {
        let record = sqlx::query!(
            r#"
              INSERT INTO question ( title, description )
              VALUES ( $1, $2 )
              RETURNING *
            "#,
            question.title,
            question.description
        )
        .fetch_one(&self.db)
        .await
        .map_err(|e| DBError::Other(Box::new(e)))?;

        Ok(QuestionDetail {
            question_uuid: record.question_uuid.to_string(),
            title: record.title,
            description: record.description,
            created_at: record.created_at.to_string(),
        })
    }

    async fn get_questions(&self) -> Result<Vec<QuestionDetail>, DBError> {
        todo!();
    }

    async fn delete_question(&self, question_uuid: QuestionId) -> Result<(), DBError> {
        todo!();
    }
}
