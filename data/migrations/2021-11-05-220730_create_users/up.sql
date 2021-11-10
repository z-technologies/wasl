CREATE TABLE users (
    id            SERIAL       PRIMARY KEY,

    username      VARCHAR(32)  NOT NULL UNIQUE,
    password_hash VARCHAR(64)  NULL,
    password_salt VARCHAR(64)  NULL,

    email         VARCHAR(64)  NOT NULL UNIQUE,
    is_active     BOOLEAN      NOT NULL DEFAULT FALSE,

    first_name    VARCHAR(32)  NULL,
    last_name     VARCHAR(32)  NULL,
    profile_photo VARCHAR(256) NULL
)
