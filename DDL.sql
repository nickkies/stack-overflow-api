-- question table
CREATE TABLE question (
    question_uuid UUID PRIMARY KEY,
    title VARCHAR(255),
    description VARCHAR(255),
    created_at TIMESTAMP
);

COMMENT ON TABLE question IS 'Question table';

COMMENT ON COLUMN question.question_uuid IS 'Generated identifier unique to each question';
COMMENT ON COLUMN question.title IS 'Title of the question';
COMMENT ON COLUMN question.description IS 'Description of the question';
COMMENT ON COLUMN question.created_at IS 'Creation timestamp of the question';

-- answer table
CREATE TABLE answer (
    answer_uuid UUID PRIMARY KEY,
    question_uuid UUID,
    content VARCHAR(255),
    created_at TIMESTAMP,
    FOREIGN KEY (question_uuid) REFERENCES question(question_uuid)
);

COMMENT ON TABLE answer IS 'Answer talbe';

COMMENT ON COLUMN answer.answer_uuid IS 'Generated identifier unique to each answer ';
COMMENT ON COLUMN answer.question_uuid IS 'Generated identifier unique to each question';
COMMENT ON COLUMN answer.content IS 'Content of the answer';
COMMENT ON COLUMN answer.created_at IS 'Creation timestamp of the answer';
