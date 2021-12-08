use crate::result::Result;
use data::diesel::BelongingToDsl;
use data::models::Group;
use data::models::User;
use data::models::UserGroup;

use data::diesel::QueryDsl;
use data::diesel::RunQueryDsl;

use data::context::DbContext;
use data::diesel::ExpressionMethods;
use data::repos::Repo;

pub struct UsersService {
    ctx: DbContext,
}

impl UsersService {
    pub fn new(ctx: DbContext) -> UsersService {
        UsersService { ctx }
    }

    pub fn get_by_username(&self, uname: &str) -> Result<User> {
        use data::schema::users::dsl::*;

        Ok(self.ctx.users().first(username.eq(uname))?)
    }

    pub fn get_by_email(&self, em: &str) -> Result<User> {
        use data::schema::users::dsl::*;

        Ok(self.ctx.users().first(email.eq(em))?)
    }

    pub fn duplicate_username(&self, uname: &str) -> Result<bool> {
        use data::diesel::dsl::*;
        use data::schema::users::dsl::*;

        Ok(select(exists(users.filter(username.eq(uname))))
            .get_result(&self.ctx.pool().get().unwrap())
            .unwrap())
    }

    pub fn duplicate_email(&self, em: &str) -> Result<bool> {
        use data::diesel::dsl::*;
        use data::schema::users::dsl::*;

        Ok(select(exists(users.filter(email.eq(em))))
            .get_result(&self.ctx.pool().get().unwrap())
            .unwrap())
    }

    pub fn get_user_groups(&self, user: &User) -> Result<Vec<Group>> {
        use data::diesel::dsl::*;
        use data::schema::groups;
        use data::schema::user_groups::dsl::*;

        let group_ids = UserGroup::belonging_to(user).select(group_id);

        Ok(groups::table
            .filter(groups::id.eq(any(group_ids)))
            .load(&self.ctx.pool().get().unwrap())
            .unwrap())
    }
}
