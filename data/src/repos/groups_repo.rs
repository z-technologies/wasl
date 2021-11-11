use data_derive;

use crate::models::group::*;

use crate::repos::{DbPool, DbPooledConnection, Repo, RepoTypes};
use crate::result;

use diesel::prelude::*;

#[derive(data_derive::Repository)]
pub struct GroupsRepo<'db> {
    pub pool: &'db DbPool,
}

impl<'a> RepoTypes for GroupsRepo<'a> {
    type Model = Group;
    type InsertModel = NewGroup<'a>;
}
