CREATE TABLE confirmations (
    id         SERIAL    PRIMARY KEY,
    user_id    SERIAL    NOT NULL REFERENCES users(id),
    otp        VARCHAR   NOT NULL,
    token      VARCHAR   NOT NULL,
    issued_at  TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL
)
