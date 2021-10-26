CREATE TABLE response (
    id uuid PRIMARY KEY,
    form_id uuid NOT NULL references form (id)
);