-- question table
CREATE TABLE IF NOT EXISTS question (
    question_uuid UUID PRIMARY KEY,
    title VARCHAR(255),
    description VARCHAR(255),
    created_at TIMESTAMP
);

-- answer table
CREATE TABLE IF NOT EXISTS answer (
    answer_uuid UUID PRIMARY KEY,
    question_uuid UUID,
    content VARCHAR(255),
    created_at TIMESTAMP,
    FOREIGN KEY (question_uuid) REFERENCES question(question_uuid)
);
