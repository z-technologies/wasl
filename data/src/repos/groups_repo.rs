use data_derive;

use crate::models::group::*;

use crate::repos::{DbPool, DbPooledConnection, Repo, RepoTypes};
use crate::result;

use diesel::prelude::*;

#[derive(Clone, data_derive::Repository)]
pub struct GroupsRepo {
    pub pool: DbPool,
}

impl RepoTypes for GroupsRepo {
    type Model = Group;
    type InsertModel = NewGroup;
}
