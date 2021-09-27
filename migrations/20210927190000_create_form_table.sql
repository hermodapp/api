DROP TABLE IF EXISTS form;

CREATE TABLE form (
       id uuid PRIMARY KEY,
       qr_code_id uuid NOT NULL REFERENCES qr_code (id),
       account_id uuid NOT NULL REFERENCES account (id)
);
