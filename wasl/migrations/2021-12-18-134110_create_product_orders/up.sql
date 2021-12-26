CREATE TYPE product_order_state as ENUM (
    'pending',
    'declined',
    'accepted'
);

CREATE TABLE product_orders (
    id             SERIAL PRIMARY KEY,

    state    product_order_state NOT NULL DEFAULT 'pending',
    quantity BIGINT              NOT NULL CHECK(quantity >= 0),

    product_id     SERIAL NOT NULL REFERENCES products(id),
    transaction_id SERIAL NOT NULL REFERENCES transactions(id),

    made_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
