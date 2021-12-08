use crate::models::KeyType;
use crate::result::Result;

use diesel::{
    associations::HasTable,
    helper_types::{Filter, Find, Limit, Select, Update},
    prelude::*,
    query_builder::{
        AsChangeset, DeleteStatement, InsertStatement, IntoUpdateTarget,
    },
    query_dsl::{
        methods::{ExecuteDsl, FilterDsl, FindDsl, LimitDsl, SelectDsl},
        LoadQuery,
    },
    r2d2::{ConnectionManager, Pool, PooledConnection},
    PgConnection, QueryDsl, RunQueryDsl,
};

pub type DbConnection = PgConnection;
pub type DbConnectionManager = ConnectionManager<DbConnection>;

pub type DbPooledConnection = PooledConnection<DbConnectionManager>;
pub type DbPool = Pool<DbConnectionManager>;

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

    fn first<E>(&self, exp: E) -> Result<M>
    where
        M::Table: FilterDsl<E>,
        Filter<M::Table, E>: LimitDsl,
        Limit<Filter<M::Table, E>>: LoadQuery<DbPooledConnection, M>,
    {
        Ok(QueryDsl::filter(M::table(), exp)
            .limit(1)
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

    fn remove(&self, item: M) -> Result<usize>
    where
        M: Identifiable,
        M::Table: FindDsl<M::Id>,
        Find<M::Table, M::Id>: IntoUpdateTarget,
        DeleteFindStatement<Find<M::Table, M::Id>>:
            ExecuteDsl<DbPooledConnection>,
    {
        Ok(diesel::delete(QueryDsl::find(M::table(), item.id()))
            .execute(&self.get_connection()?)?)
    }

    fn update(&self, item: M) -> Result<M>
    where
        M: IntoUpdateTarget + AsChangeset<Target = M::Table>,
        Update<M, M>: LoadQuery<DbPooledConnection, M>,
    {
        Ok(diesel::update(item.clone())
            .set(item)
            .get_result(&self.get_connection()?)?)
    }

    fn filter<E>(&self, exp: E) -> Result<Vec<M>>
    where
        M::Table: FilterDsl<E>,
        Filter<M::Table, E>: LoadQuery<DbPooledConnection, M>,
    {
        Ok(QueryDsl::filter(M::table(), exp).load(&self.get_connection()?)?)
    }

    fn get_connection(&self) -> Result<DbPooledConnection>;
}

type DeleteFindStatement<F> = DeleteStatement<
    <F as HasTable>::Table,
    <F as IntoUpdateTarget>::WhereClause,
>;
