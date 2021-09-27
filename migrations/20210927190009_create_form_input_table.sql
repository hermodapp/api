DROP TABLE IF EXISTS form_input;

CREATE TABLE form_input (
       id UUID PRIMARY KEY,
       form_id UUID NOT NULL REFERENCES form (id),
       type TEXT NOT NULL
);
