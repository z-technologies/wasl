use crate::models::group::*;
use crate::repos::{DbPool, DbPooledConnection, Repo, RepoTypes};
use crate::result::{DataError, DataResult};

use data_derive::Repository;
use diesel::prelude::*;

#[derive(Clone, Repository)]
pub struct GroupsRepo {
    pub pool: DbPool,
}

impl RepoTypes for GroupsRepo {
    type Model = Group;
    type InsertModel = NewGroup;
}
