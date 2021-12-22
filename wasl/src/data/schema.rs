table! {
    use diesel::sql_types::*;
    use crate::data::models::*;

    confirmations (id) {
        id -> Int4,
        otp -> Varchar,
        token -> Varchar,
        user_id -> Int4,
        issued_at -> Timestamptz,
        expires_at -> Timestamptz,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::data::models::*;

    financial_record_verifications (id) {
        id -> Int4,
        outcome -> Financial_record_verification_outcome,
        financial_record_id -> Int4,
        verified_by -> Int4,
        verified_at -> Timestamptz,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::data::models::*;

    financial_records (id) {
        id -> Int4,
        amount -> Numeric,
        made_by -> Int4,
        made_at -> Timestamptz,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::data::models::*;

    groups (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::data::models::*;

    product_orders (id) {
        id -> Int4,
        product_id -> Int4,
        transaction_id -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::data::models::*;

    products (id) {
        id -> Int4,
        title -> Varchar,
        description -> Varchar,
        price -> Numeric,
        available_quantity -> Int8,
        user_id -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::data::models::*;

    service_reservations (id) {
        id -> Int4,
        reservation_begin -> Timestamptz,
        reservation_end -> Timestamptz,
        service_id -> Int4,
        transaction_id -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::data::models::*;

    services (id) {
        id -> Int4,
        title -> Varchar,
        description -> Varchar,
        price -> Numeric,
        available_begin -> Nullable<Time>,
        available_end -> Nullable<Time>,
        user_id -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::data::models::*;

    transaction_confirmations (id) {
        id -> Int4,
        outcome -> Transaction_confirmation_outcome,
        transaction_id -> Int4,
        confirmed_at -> Timestamptz,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::data::models::*;

    transactions (id) {
        id -> Int4,
        amount -> Numeric,
        sender -> Int4,
        receiver -> Int4,
        made_at -> Timestamptz,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::data::models::*;

    user_groups (id) {
        id -> Int4,
        user_id -> Int4,
        group_id -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::data::models::*;

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
joinable!(financial_record_verifications -> financial_records (financial_record_id));
joinable!(financial_record_verifications -> users (verified_by));
joinable!(financial_records -> users (made_by));
joinable!(product_orders -> products (product_id));
joinable!(product_orders -> transactions (transaction_id));
joinable!(products -> users (user_id));
joinable!(service_reservations -> services (service_id));
joinable!(service_reservations -> transactions (transaction_id));
joinable!(services -> users (user_id));
joinable!(transaction_confirmations -> transactions (transaction_id));
joinable!(user_groups -> groups (group_id));
joinable!(user_groups -> users (user_id));

allow_tables_to_appear_in_same_query!(
    confirmations,
    financial_record_verifications,
    financial_records,
    groups,
    product_orders,
    products,
    service_reservations,
    services,
    transaction_confirmations,
    transactions,
    user_groups,
    users,
);
