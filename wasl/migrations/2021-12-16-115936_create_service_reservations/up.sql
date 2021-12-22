CREATE TABLE service_reservations (
    id                SERIAL PRIMARY KEY,

    reservation_begin TIMESTAMP WITH TIME ZONE NOT NULL,
    reservation_end   TIMESTAMP WITH TIME ZONE NOT NULL,

    made_by        SERIAL NOT NULL REFERENCES users(id),
    service_id     SERIAL NOT NULL REFERENCES services(id),
    transaction_id SERIAL NOT NULL REFERENCES transactions(id)
);
