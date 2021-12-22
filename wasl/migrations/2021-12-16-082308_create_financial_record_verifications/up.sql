CREATE TYPE financial_record_verification_outcome AS ENUM(
    'rejected',
    'verified'
);

CREATE TABLE financial_record_verifications (
    id SERIAL PRIMARY KEY,

    outcome financial_record_verification_outcome NOT NULL,
    financial_record_id SERIAL NOT NULL REFERENCES financial_records(id),
    verified_by         SERIAL NOT NULL REFERENCES users(id)

    verified_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
);
