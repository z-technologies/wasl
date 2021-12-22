CREATE TABLE product_orders (
    id             SERIAL PRIMARY KEY,

    product_id     SERIAL NOT NULL REFERENCES products(id),
    transaction_id SERIAL NOT NULL REFERENCES transactions(id)
);
