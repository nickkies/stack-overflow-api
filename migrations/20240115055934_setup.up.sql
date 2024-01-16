-- question table
CREATE TABLE IF NOT EXISTS question (
    question_uuid UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(255) NOT NULL,
    description VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);


-- answer table
CREATE TABLE IF NOT EXISTS answer (
    answer_uuid UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    question_uuid UUID NOT NULL REFERENCES question (question_uuid) ON DELETE CASCADE,
    content VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
