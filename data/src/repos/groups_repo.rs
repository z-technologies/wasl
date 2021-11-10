use data_derive;

use crate::models::group::*;
use crate::repos::{DbConnection, Repo, RepoTypes};
use crate::result;

use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

#[derive(data_derive::Repository)]
pub struct GroupsRepo<'a> {
    pub db: &'a DbConnection,
}

impl<'a> RepoTypes for GroupsRepo<'a> {
    type Model = Group;
    type InsertModel = NewGroup<'a>;
}
