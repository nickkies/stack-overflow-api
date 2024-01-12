pub struct Question {
    pub title: String,
    pub description: String,
}

pub struct QuestionDetail {
    pub question_uuid: String,
    pub title: String,
    pub description: String,
    pub created_at: String,
}

pub struct QuestionId {
    pub question_uuid: String,
}

pub struct Answer {
    pub question_uuid: String,
    pub content: String,
}

pub struct AnswerDetail {
    pub answer_uuid: String,
    pub question_uuid: String,
    pub content: String,
    pub created_at: String,
}

pub struct AnswerId {
    pub answer_uuid: String,
}
