CREATE TABLE transactions (
    id          SERIAL                   PRIMARY KEY,
    amount      NUMERIC                  NOT NULL,
    sender      SERIAL                   NOT NULL REFERENCES users(id),
    receiver    SERIAL                   NOT NULL REFERENCES users(id),
    made_at     TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
)
