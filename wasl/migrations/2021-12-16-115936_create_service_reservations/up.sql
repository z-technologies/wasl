CREATE TABLE service_reservations (
    id                SERIAL PRIMARY KEY,

    reservation_begin TIMESTAMP WITH TIME ZONE NOT NULL,
    reservation_end   TIMESTAMP WITH TIME ZONE NOT NULL,

    service_id     SERIAL NOT NULL REFERENCES services(id),
    transaction_id SERIAL NOT NULL REFERENCES transactions(id)
);
