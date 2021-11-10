table! {
    admins (id) {
        id -> Int4,
        username -> Varchar,
        password_hash -> Varchar,
        password_salt -> Varchar,
        first_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
    }
}

table! {
    groups (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password_hash -> Nullable<Varchar>,
        password_salt -> Nullable<Varchar>,
        email -> Varchar,
        is_active -> Bool,
        first_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
        profile_photo -> Nullable<Varchar>,
        is_provider -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    admins,
    groups,
    users,
);
