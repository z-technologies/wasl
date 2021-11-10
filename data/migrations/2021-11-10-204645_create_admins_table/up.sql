CREATE TABLE admins (
    id            SERIAL       PRIMARY KEY,

    username      VARCHAR(32)  NOT NULL,
    password_hash VARCHAR(64)  NOT NULL,
    password_salt VARCHAR(64)  NOT NULL,

    first_name    VARCHAR(32)  NULL,
    last_name     VARCHAR(32)  NULL
)
