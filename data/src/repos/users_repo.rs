use data_derive;

use crate::models::user::{NewUser, User};
use crate::repos::{DbConnection, Repo, RepoTypes};
use crate::result;

use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

#[derive(data_derive::Repository)]
pub struct UsersRepo<'a> {
    pub db: &'a DbConnection,
}

impl<'a> RepoTypes for UsersRepo<'a> {
    type Model = User;
    type InsertModel = NewUser<'a>;
}
