DROP TABLE IF EXISTS forgotten_password_request; 

CREATE TABLE forgotten_password_request (
    id UUID PRIMARY KEY,
    account_id UUID NOT NULL REFERENCES account (id),
    created_at TIMESTAMP NOT NULL
);