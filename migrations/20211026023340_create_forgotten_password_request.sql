CREATE TABLE forgotten_password_request (
    id uuid PRIMARY KEY,
    account_id uuid NOT NULL REFERENCES account (id),
    created_at time NOT NULL 
);