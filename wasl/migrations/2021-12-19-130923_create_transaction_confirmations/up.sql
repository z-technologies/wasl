CREATE TYPE transaction_confirmation_outcome AS ENUM(
    'declined',
    'confirmed'
);

CREATE TABLE transaction_confirmations (
    id      SERIAL                           PRIMARY KEY,

    outcome transaction_confirmation_outcome NOT NULL,
    transaction_id SERIAL                    NOT NULL REFERENCES transactions(id)

    confirmed_at   TIMESTAMP WITH TIME ZONE  NOT NULL DEFAULT CURRENT_TIMESTAMP,
);
