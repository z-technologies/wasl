use crate::models::model::Model;

#[derive(Clone, Debug, Model, Queryable)]
pub struct User {
    id: u64,

    username: String,
    password_hash: Option<String>,
    password_salt: Option<String>,

    email: String,
    is_active: bool,

    first_name: String,
    last_name: String,
    profile_photo: String,
}
