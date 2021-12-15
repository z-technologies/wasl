CREATE TYPE financial_record_state AS ENUM(
    'pending',
    'rejected',
    'verified'
);

CREATE TABLE financial_records (
    id          SERIAL                   PRIMARY KEY,
    amount      NUMERIC                  NOT NULL,
    state       financial_record_state   NOT NULL DEFAULT 'pending',
    signature   VARCHAR                  NOT NULL,
    made_by     SERIAL                   NOT NULL REFERENCES users(id),
    made_at     TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    verified_by SERIAL                   REFERENCES users(id)
);
