CREATE TABLE services (
    id              SERIAL  PRIMARY KEY,
    title           VARCHAR NOT NULL,
    description     VARCHAR NOT NULL,
    price           NUMERIC NOT NULL,
    available_begin TIME    NULL,
    available_end   TIME    NULL,
    user_id         SERIAL  NOT NULL REFERENCES users(id)
);
