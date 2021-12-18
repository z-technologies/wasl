CREATE TABLE products (
    id                 SERIAL  PRIMARY KEY,
    title              VARCHAR NOT NULL,
    description        VARCHAR NOT NULL,
    price              NUMERIC NOT NULL CHECK(price >= 0),
    available_quantity BIGINT  NOT NULL CHECK(available_quantity >= 0),
    user_id            SERIAL  NOT NULL REFERENCES users(id)
);
