mod questions_tests {
    use sqlx::PgPool;

    use crate::{
        models::{DBError, Question},
        persistance::question_dao::{QuestionDao, QuestionDaoImpl},
    };

    #[sqlx::test]
    async fn create_question_should_fail_if_database_error_occurs(
        pool: PgPool,
    ) -> Result<(), String> {
        let dao = QuestionDaoImpl::new(pool.clone());

        pool.close().await;

        let result = dao
            .create_question(Question {
                title: "test title".to_string(),
                description: "test description".to_string(),
            })
            .await;

        if result.is_ok() {
            return Err(format!(
                "Expected as error but got the following result: {:?}",
                result.unwrap()
            ));
        }

        if let Err(DBError::InvalidUUID(_)) = result {
            Ok(())
        } else {
            Err(format!(
                "Expected an invalid UUID error but got the following error: {:?}",
                result.err()
            ))
        }
    }

    #[sqlx::test]
    async fn create_question_should_succeed(pool: PgPool) -> Result<(), String> {
        let dao = QuestionDaoImpl::new(pool);

        let result = dao
            .create_question(Question {
                title: "test title".to_string(),
                description: "test description".to_string(),
            })
            .await
            .map_err(|e| format!("{e:?}"))?;

        if result.title != "test title".to_string()
            || result.description != "test description".to_string()
        {
            Err("Incorrect title or description".to_string())
        } else {
            Ok(())
        }
    }
}

mod answer_tests {}
