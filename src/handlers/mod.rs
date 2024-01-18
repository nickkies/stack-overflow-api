use crate::{models::*, persistance::question_dao::QuestionDao};
use rocket::{serde::json::Json, State};

mod handlers_inner;

#[post("/question", data = "<question>")]
pub async fn create_question(
    question: Json<Question>,
    question_dao: &State<Box<dyn QuestionDao + Sync + Send>>,
) -> Json<QuestionDetail> {
    Json(
        handlers_inner::create_question(question.0, question_dao.inner())
            .await
            .unwrap(),
    )
}

#[get("/questions")]
pub async fn read_questions() -> Json<Vec<QuestionDetail>> {
    Json(vec![QuestionDetail {
        question_uuid: "d347261c-3f0e-42d2-8706-5ef9f1b96725".to_string(),
        title: "Newly Created Question".to_string(),
        description: "My Description".to_string(),
        created_at: "2024-01-01 00:00:00.000000".to_string(),
    }])
}

#[delete("/question", data = "<question_uuid>")]
pub async fn delete_question(question_uuid: Json<QuestionId>) {
    ()
}

#[post("/answer", data = "<answer>")]
pub async fn create_answer(answer: Json<Answer>) -> Json<AnswerDetail> {
    Json(AnswerDetail {
        answer_uuid: "a1a14a9c-ab9e-481b-8120-67f675531ed2".to_string(),
        question_uuid: answer.question_uuid.to_string(),
        content: answer.content.to_string(),
        created_at: "2024-01-01 00:00:00.000000".to_string(),
    })
}

#[get("/answers", data = "<question_uuid>")]
pub async fn read_answers(question_uuid: Json<QuestionId>) -> Json<Vec<AnswerDetail>> {
    Json(vec![AnswerDetail {
        answer_uuid: "a1a14a9c-ab9e-481b-8120-67f675531ed2".to_string(),
        question_uuid: question_uuid.question_uuid.to_string(),
        content: "test question".to_string(),
        created_at: "2024-01-01 00:00:00.000000".to_string(),
    }])
}

#[delete("/answer", data = "<answer_uuid>")]
pub async fn delete_answer(answer_uuid: Json<AnswerId>) {
    ()
}
