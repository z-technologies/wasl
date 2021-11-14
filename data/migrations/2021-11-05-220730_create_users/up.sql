CREATE TABLE users (
    id            SERIAL        PRIMARY KEY,

    username      VARCHAR(32)   NOT NULL UNIQUE,
    email         VARCHAR(64)   NOT NULL UNIQUE,
    password_hash VARCHAR(128)  NULL,

    first_name    VARCHAR(32)   NULL,
    last_name     VARCHAR(32)   NULL,
    profile_photo VARCHAR(256)  NULL,

    is_provider   BOOLEAN       NOT NULL,
    is_active     BOOLEAN       NOT NULL DEFAULT FALSE,

    created_at    TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
)
