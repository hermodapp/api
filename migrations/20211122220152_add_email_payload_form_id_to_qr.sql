ALTER TABLE qr_code
ADD email VARCHAR(320),
ADD payload VARCHAR(320),
ADD form_id uuid REFERENCES form (id);
