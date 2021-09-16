ALTER TABLE account RENAME COLUMN user_id TO id;

CREATE TABLE qr_code (
    id uuid PRIMARY KEY,
    account_id uuid NOT NULL REFERENCES account (id),
    slug TEXT NOT NULL,
    generation_data TEXT NOT NULL
);