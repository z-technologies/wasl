table! {
    confirmations (id) {
        id -> Int4,
        user_id -> Int4,
        otp -> Varchar,
        token -> Varchar,
        issued_at -> Timestamptz,
        expires_at -> Timestamptz,
    }
}

table! {
    groups (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    user_groups (id) {
        id -> Int4,
        user_id -> Int4,
        group_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password_hash -> Varchar,
        is_active -> Bool,
        first_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
        profile_photo -> Nullable<Varchar>,
        cached_balance -> Float8,
        created_at -> Timestamptz,
    }
}

joinable!(confirmations -> users (user_id));
joinable!(user_groups -> groups (group_id));
joinable!(user_groups -> users (user_id));

allow_tables_to_appear_in_same_query!(
    confirmations,
    groups,
    user_groups,
    users,
);
