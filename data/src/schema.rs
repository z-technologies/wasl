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
    }
}
