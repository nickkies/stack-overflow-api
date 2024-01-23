mod test_util {
    use dotenvy::dotenv;
    use sqlx::{postgres::PgPoolOptions, PgPool};

    pub async fn create_test_pool() -> PgPool {
        dotenv().ok();

        PgPoolOptions::new()
            .max_connections(1)
            .connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL must be set."))
            .await
            .expect("Failed to create Postgres connection pool!")
    }
}

mod questions_tests {
    use crate::{
        models::{Answer, DBError, Question},
        persistance::{
            answer_dao::{AnswerDao, AnswerDaoImpl},
            question_dao::{QuestionDao, QuestionDaoImpl},
        },
    };

    use super::test_util::create_test_pool;

    #[tokio::test]
    async fn create_answer_should_fail_with_malformed_uuid() -> Result<(), String> {
        let pool = create_test_pool().await;
        let dao = AnswerDaoImpl::new(pool);
        let result = dao
            .create_answer(Answer {
                question_uuid: "malformed".to_string(),
                content: "test content".to_string(),
            })
            .await;

        if result.is_ok() {
            return Err(format!(
                "Expected an error got the following result: {:?}",
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

    #[tokio::test]
    async fn create_answer_should_fail_with_non_existent_uuid() -> Result<(), String> {
        let pool = create_test_pool().await;
        let dao = AnswerDaoImpl::new(pool);
        let result = dao
            .create_answer(Answer {
                question_uuid: "aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa".to_string(),
                content: "test content".to_string(),
            })
            .await;

        if result.is_ok() {
            return Err(format!(
                "Expected an error but got the following result: {:?}",
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

    async fn create_answer_should_fail_if_database_error_occcurs() -> Result<(), String> {
        let pool = create_test_pool().await;
        let dao = AnswerDaoImpl::new(pool.clone());

        pool.close().await;

        let result = dao
            .create_answer(Answer {
                question_uuid: "b068cd2f-edac-479e-98f1-c5f91008dcbd".to_string(),
                content: "test content".to_string(),
            })
            .await;

        if result.is_ok() {
            return Err(format!(
                "Expected an error but got the following result: {:?}",
                result.unwrap()
            ));
        }

        if let Err(DBError::Other(_)) = result {
            Ok(())
        } else {
            Err(format!(
                "Expected an Other error but got the following error: {:?}",
                result.err()
            ))
        }
    }

    #[tokio::test]
    async fn create_question_should_fail_if_database_error_occurs() -> Result<(), String> {
        let pool = create_test_pool().await;
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

        if let Err(DBError::Other(_)) = result {
            Ok(())
        } else {
            Err(format!(
                "Expected an Other error but got the following error: {:?}",
                result.err()
            ))
        }
    }

    #[tokio::test]
    async fn create_question_should_succeed() -> Result<(), String> {
        let pool = create_test_pool().await;
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

    #[tokio::test]
    async fn get_questions_should_fail_if_database_error_occurs() -> Result<(), String> {
        let pool = create_test_pool().await;
        let dao = QuestionDaoImpl::new(pool.clone());

        pool.close().await;

        let result = dao.get_questions().await;

        if result.is_ok() {
            return Err(format!(
                "Expected an error but got the following result: {:?}",
                result.unwrap()
            ));
        }

        if let Err(DBError::Other(_)) = result {
            Ok(())
        } else {
            Err(format!(
                "Expected an error but got the following result: {:?}",
                result.err()
            ))
        }
    }

    #[tokio::test]
    async fn get_questions_should_succeed() -> Result<(), String> {
        let pool = create_test_pool().await;
        let dao = QuestionDaoImpl::new(pool);
        let result = dao
            .create_question(Question {
                title: "test title".to_string(),
                description: "test description".to_string(),
            })
            .await
            .map_err(|e| format!("{e:?}"))?;

        let results = dao.get_questions().await.map_err(|e| format!("{e:?}"))?;

        if results.len() != 1 {
            Err("incorrect number of results returned.".to_string())
        } else if results.get(0).unwrap().question_uuid != result.question_uuid {
            Err("Incorrect question returned.".to_string())
        } else {
            Ok(())
        }
    }

    #[tokio::test]
    async fn delete_qeustion_should_fail_with_malformed_uuid() -> Result<(), String> {
        let pool = create_test_pool().await;
        let dao = QuestionDaoImpl::new(pool);
        let result = dao.delete_question("malformed".to_string()).await;

        if result.is_ok() {
            return Err(format!(
                "Expected an error but got the following result: {:?}",
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

    #[tokio::test]
    async fn delete_question_should_fail_if_database_error_occurs() -> Result<(), String> {
        let pool = create_test_pool().await;
        let dao = QuestionDaoImpl::new(pool.clone());

        pool.close().await;

        let result = dao
            .delete_question("c4d24be8-8655-414f-81f0-8cf3ff11245a".to_string())
            .await;

        if let Err(DBError::Other(_)) = result {
            Ok(())
        } else {
            Err(format!(
                "Expected an Other error but got the following error: {:?}",
                result.err()
            ))
        }
    }

    #[tokio::test]
    async fn delete_question_should_succeed() -> Result<(), String> {
        let pool = create_test_pool().await;
        let dao = QuestionDaoImpl::new(pool);
        let result = dao
            .create_question(Question {
                title: "test title".to_string(),
                description: "test description".to_string(),
            })
            .await
            .map_err(|e| format!("{e:?}"))?;

        dao.delete_question(result.question_uuid)
            .await
            .map_err(|e| format!("{e:?}"))?;

        let results = dao.get_questions().await.map_err(|e| format!("{e:?}"))?;

        if results.len() == 0 {
            Ok(())
        } else {
            Err("Qeustion was not deleted".to_string())
        }
    }
}

mod answer_tests {}
