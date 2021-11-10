CREATE TABLE admin_groups (
    id       SERIAL PRIMARY KEY,
    admin_id SERIAL REFERENCES admins(id),
    group_id SERIAL REFERENCES groups(id)
)
