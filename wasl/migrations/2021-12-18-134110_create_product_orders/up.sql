CREATE TABLE product_orders (
    id             SERIAL PRIMARY KEY,

    made_by        SERIAL NOT NULL REFERENCES users(id),
    product_id     SERIAL NOT NULL REFERENCES products(id),
    transaction_id SERIAL NOT NULL REFERENCES transactions(id)
);
