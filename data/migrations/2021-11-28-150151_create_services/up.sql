CREATE TABLE services (
    id              SERIAL  PRIMARY KEY,
    title           VARCHAR NOT NULL,
    description     VARCHAR NOT NULL,
    available_begin TIME    NULL,
    available_end   TIME    NULL
);