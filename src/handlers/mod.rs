use crate::models::*;
use rocket::serde::json::Json;

#[post("/question", data = "<question>")]
pub async fn create_question(question: Json<Question>) -> Json<QuestionDetail> {
    Json(QuestionDetail {
        question_uuid: "d347261c-3f0e-42d2-8706-5ef9f1b96725".to_string(),
        title: question.title.to_string(),
        description: question.description.to_string(),
        created_at: "2024-01-01 00:00:00.000000".to_string(),
    })
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
