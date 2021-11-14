CREATE TABLE user_groups (
    id       SERIAL PRIMARY KEY,

    user_id  SERIAL NOT NULL    REFERENCES users(id),
    group_id SERIAL NOT NULL    REFERENCES groups(id)
)
