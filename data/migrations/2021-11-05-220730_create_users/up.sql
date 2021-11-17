CREATE TABLE users (
    id             SERIAL        PRIMARY KEY,

    username       VARCHAR       NOT NULL UNIQUE,
    email          VARCHAR       NOT NULL UNIQUE,
    password_hash  VARCHAR(128)  NULL,
    is_active      BOOLEAN       NOT NULL DEFAULT FALSE,

    first_name     VARCHAR       NULL,
    last_name      VARCHAR       NULL,
    profile_photo  VARCHAR       NULL,

    cached_balance FLOAT8        NOT NULL DEFAULT 0,
    created_at     TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
)
