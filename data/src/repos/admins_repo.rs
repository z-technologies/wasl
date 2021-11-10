use data_derive;

use crate::models::admin::*;
use crate::models::group::*;

use crate::repos::{DbConnection, Repo, RepoTypes};
use crate::result;

use diesel::pg::expression::dsl::any;
use diesel::prelude::*;

#[derive(data_derive::Repository)]
pub struct AdminsRepo<'a> {
    pub db: &'a DbConnection,
}

impl<'a> RepoTypes for AdminsRepo<'a> {
    type Model = Admin;
    type InsertModel = NewAdmin<'a>;
}

impl<'a> AdminsRepo<'a> {
    pub fn get_admin_groups(&self, admin: &Admin) -> result::Result<Vec<Group>> {
        use crate::schema::admin_groups::dsl::*;
        use crate::schema::groups;

        let group_ids = AdminGroup::belonging_to(admin).select(group_id);

        Ok(groups::table
            .filter(groups::id.eq(any(group_ids)))
            .load::<Group>(self.db)?)
    }
}
