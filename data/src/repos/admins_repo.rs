use data_derive;

use crate::models::admin::*;
use crate::repos::{DbConnection, Repo, RepoTypes};
use crate::result;

use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

#[derive(data_derive::Repository)]
pub struct AdminsRepo<'a> {
    pub db: &'a DbConnection,
}

impl<'a> RepoTypes for AdminsRepo<'a> {
    type Model = Admin;
    type InsertModel = NewAdmin<'a>;
}
