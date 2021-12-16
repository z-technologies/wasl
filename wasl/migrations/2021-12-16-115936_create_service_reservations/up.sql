CREATE TABLE service_reservations (
    id                SERIAL PRIMARY KEY,
    made_by           SERIAL NOT NULL REFERENCES services(id),
    service_id        SERIAL NOT NULL REFERENCES services(id),

    reservation_begin TIMESTAMP WITH TIME ZONE NOT NULL,
    reservation_end   TIMESTAMP WITH TIME ZONE NOT NULL
);
