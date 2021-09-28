DROP TABLE IF EXISTS feedback;

CREATE TABLE feedback (
       id UUID PRIMARY KEY,
       form_input_id UUID NOT NULL REFERENCES form_input (id),
       payload JSONB NOT NULL
);
