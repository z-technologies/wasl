table! {
    admin_groups (id) {
        id -> Int4,
        admin_id -> Int4,
        group_id -> Int4,
    }
}

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

joinable!(admin_groups -> admins (admin_id));
joinable!(admin_groups -> groups (group_id));

allow_tables_to_appear_in_same_query!(
    admin_groups,
    admins,
    groups,
    users,
);
