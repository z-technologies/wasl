use crate::models::KeyType;
use crate::repos::DbPooledConnection;
use crate::result::Result;
use diesel::query_builder::UpdateStatement;

use diesel::{
    associations::HasTable,
    helper_types::{Find, Select, Update},
    prelude::*,
    query_builder::{
        AsChangeset, DeleteStatement, InsertStatement, IntoUpdateTarget,
    },
    query_dsl::{
        methods::{ExecuteDsl, FindDsl, SelectDsl},
        LoadQuery,
    },
    QueryDsl, RunQueryDsl,
};

pub trait Repo<M>
where
    M: Clone + HasTable,
{
    fn get(&self, id: KeyType) -> Result<M>
    where
        M::Table: FindDsl<KeyType>,
        Find<M::Table, KeyType>: LoadQuery<DbPooledConnection, M>,
    {
        Ok(QueryDsl::find(M::table(), id)
            .get_result(&self.get_connection()?)?)
    }

    fn get_all(&self) -> Result<Vec<M>>
    where
        M::Table: SelectDsl<<M::Table as Table>::AllColumns>,
        Select<M::Table, <M::Table as Table>::AllColumns>:
            LoadQuery<DbPooledConnection, M>,
    {
        Ok(QueryDsl::select(M::table(), M::Table::all_columns())
            .load(&self.get_connection()?)?)
    }

    fn add<N>(&self, item: N) -> Result<M>
    where
        N: Insertable<M::Table>,
        InsertStatement<M::Table, N::Values>: LoadQuery<DbPooledConnection, M>,
    {
        Ok(diesel::insert_into(M::table())
            .values(item)
            .get_result(&self.get_connection()?)?)
    }

    fn remove(&self, id: KeyType) -> Result<usize>
    where
        M::Table: FindDsl<KeyType>,
        Find<M::Table, KeyType>: IntoUpdateTarget,
        DeleteFindStatement<Find<M::Table, KeyType>>:
            ExecuteDsl<DbPooledConnection>,
    {
        Ok(diesel::delete(QueryDsl::find(M::table(), id))
            .execute(&self.get_connection()?)?)
    }

    fn update(&self, item: M) -> Result<M>
    where
        M: IntoUpdateTarget,
        M: IntoUpdateTarget + AsChangeset<Target = M::Table>,
        Update<M, M>: LoadQuery<DbPooledConnection, M>,
    {
        Ok(diesel::update(item.clone())
            .set(item)
            .get_result(&self.get_connection()?)?)
    }

    fn get_connection(&self) -> Result<DbPooledConnection>;
}

type DeleteFindStatement<F> = DeleteStatement<
    <F as HasTable>::Table,
    <F as IntoUpdateTarget>::WhereClause,
>;
