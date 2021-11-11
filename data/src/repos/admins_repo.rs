use data_derive;

use crate::models::admin::*;
use crate::models::group::*;

use crate::repos::{DbPool, DbPooledConnection, Repo, RepoTypes};
use crate::result;

use diesel::pg::expression::dsl::any;
use diesel::prelude::*;

#[derive(Clone, data_derive::Repository)]
pub struct AdminsRepo {
    pub pool: DbPool,
}

impl RepoTypes for AdminsRepo {
    type Model = Admin;
    type InsertModel = NewAdmin;
}

impl AdminsRepo {
    pub fn get_admin_groups(
        &self,
        admin: &Admin,
    ) -> result::Result<Vec<Group>> {
        use crate::schema::admin_groups::dsl::*;
        use crate::schema::groups;

        let group_ids = AdminGroup::belonging_to(admin).select(group_id);

        Ok(groups::table
            .filter(groups::id.eq(any(group_ids)))
            .load::<Group>(&self.get_connection()?)?)
    }
}
