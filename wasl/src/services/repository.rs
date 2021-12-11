use crate::data::connection::*;
use crate::data::models::KeyType;
use crate::result::Result;

use diesel::{
    associations::HasTable,
    helper_types::{Filter, Find, Limit, Select, Update},
    prelude::*,
    query_builder::{AsChangeset, InsertStatement, IntoUpdateTarget},
    query_dsl::{
        methods::{FilterDsl, FindDsl, LimitDsl, SelectDsl},
        LoadQuery,
    },
    QueryDsl, RunQueryDsl,
};

type DbConnection = <PostgresConnection as DatabaseConnection>::Conn;

pub(crate) trait Repo<M>
where
    M: Clone + HasTable,
{
    fn get(&self, id: KeyType) -> Result<M>
    where
        M::Table: FindDsl<KeyType>,
        Find<M::Table, KeyType>: LoadQuery<DbConnection, M>,
    {
        Ok(QueryDsl::find(M::table(), id)
            .get_result(&self.get_connection()?)?)
    }

    fn first<E>(&self, exp: E) -> Result<M>
    where
        M::Table: FilterDsl<E>,
        Filter<M::Table, E>: LimitDsl,
        Limit<Filter<M::Table, E>>: LoadQuery<DbConnection, M>,
    {
        Ok(QueryDsl::filter(M::table(), exp)
            .limit(1)
            .get_result(&self.get_connection()?)?)
    }

    fn get_all(&self) -> Result<Vec<M>>
    where
        M::Table: SelectDsl<<M::Table as Table>::AllColumns>,
        Select<M::Table, <M::Table as Table>::AllColumns>:
            LoadQuery<DbConnection, M>,
    {
        Ok(QueryDsl::select(M::table(), M::Table::all_columns())
            .load(&self.get_connection()?)?)
    }

    fn create<'a, N>(&self, item: &'a N) -> Result<M>
    where
        &'a N: Insertable<M::Table>,
        InsertStatement<M::Table, <&'a N as Insertable<M::Table>>::Values>:
            LoadQuery<DbConnection, M>,
    {
        Ok(diesel::insert_into(M::table())
            .values(item)
            .get_result(&self.get_connection()?)?)
    }

    fn update(&self, item: M) -> Result<M>
    where
        M: IntoUpdateTarget + AsChangeset<Target = M::Table>,
        Update<M, M>: LoadQuery<DbConnection, M>,
    {
        Ok(diesel::update(item.clone())
            .set(item)
            .get_result(&self.get_connection()?)?)
    }

    fn filter<E>(&self, exp: E) -> Result<Vec<M>>
    where
        M::Table: FilterDsl<E>,
        Filter<M::Table, E>: LoadQuery<DbConnection, M>,
    {
        Ok(QueryDsl::filter(M::table(), exp).load(&self.get_connection()?)?)
    }

    fn get_connection(&self) -> Result<DbConnection>;
}
