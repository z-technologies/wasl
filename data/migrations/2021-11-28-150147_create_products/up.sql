CREATE TABLE products (
    id                 SERIAL  PRIMARY KEY,
    title              VARCHAR NOT NULL,
    description        VARCHAR NOT NULL,
    available_quantity INTEGER NOT NULL,
    user_id            SERIAL  NOT NULL REFERENCES users(id)
);
