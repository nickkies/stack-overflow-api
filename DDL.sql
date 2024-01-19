DROP TABLE IF EXISTS question, answer;

-- question table
CREATE TABLE IF NOT EXISTS question (
    question_uuid UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(255) NOT NULL,
    description VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

COMMENT ON TABLE question IS 'Question table';

COMMENT ON COLUMN question.question_uuid IS 'Generated identifier unique to each question';
COMMENT ON COLUMN question.title IS 'Title of the question';
COMMENT ON COLUMN question.description IS 'Description of the question';
COMMENT ON COLUMN question.created_at IS 'Creation timestamp of the question';

-- answer table
CREATE TABLE IF NOT EXISTS answer (
    answer_uuid UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    question_uuid UUID NOT NULL REFERENCES question (question_uuid) ON DELETE CASCADE,
    content VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

COMMENT ON TABLE answer IS 'Answer table';

COMMENT ON COLUMN answer.answer_uuid IS 'Generated identifier unique to each answer ';
COMMENT ON COLUMN answer.question_uuid IS 'Generated identifier unique to each question';
COMMENT ON COLUMN answer.content IS 'Content of the answer';
COMMENT ON COLUMN answer.created_at IS 'Creation timestamp of the answer';
