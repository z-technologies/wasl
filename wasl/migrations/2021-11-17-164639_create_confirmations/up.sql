CREATE TABLE confirmations (
    id         SERIAL    PRIMARY KEY,

    otp        VARCHAR   NOT NULL,
    token      VARCHAR   NOT NULL,
    user_id    SERIAL    NOT NULL REFERENCES users(id),

    issued_at  TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL
)
