use std::convert::TryInto;

use diesel::{ExpressionMethods, JoinOnDsl, NullableExpressionMethods, QueryDsl};

use crate::schema;

#[derive(Debug, Queryable)]
pub struct Contest {
    pub contest_id: i32,
    pub title: String,
    pub contest_type: String,
    pub start_time: i64,
    pub options: i32,
    pub data: String,
    pub info: String,
    pub visible: bool,
    pub author_id: i32,
    pub allow_languages: String,
}

type AllColumns = (
    schema::contests::columns::contest_id,
    schema::contests::columns::title,
    schema::contests::columns::contest_type,
    schema::contests::columns::start_time,
    schema::contests::columns::options,
    schema::contests::columns::data,
    schema::contests::columns::info,
    schema::contests::columns::visible,
    schema::contests::columns::author_id,
    schema::contests::columns::allow_languages,
);

const ALL_COLUMNS: AllColumns = (
    schema::contests::columns::contest_id,
    schema::contests::columns::title,
    schema::contests::columns::contest_type,
    schema::contests::columns::start_time,
    schema::contests::columns::options,
    schema::contests::columns::data,
    schema::contests::columns::info,
    schema::contests::columns::visible,
    schema::contests::columns::author_id,
    schema::contests::columns::allow_languages,
);

impl Contest {
    pub fn select() -> diesel::dsl::Select<schema::contests::table, AllColumns> {
        schema::contests::dsl::contests.select(ALL_COLUMNS)
    }

    pub fn select_registered(
        user_id: u32,
    ) -> diesel::dsl::Filter<
        diesel::dsl::Select<
            diesel::dsl::InnerJoin<schema::contest_users::table, schema::contests::table>,
            AllColumns,
        >,
        diesel::expression::operators::Eq<
            schema::contest_users::columns::user_id,
            diesel::expression::bound::Bound<diesel::sql_types::Integer, i32>,
        >,
    > {
        let user_id: i32 = user_id.try_into().expect("DOTS database is inconsistent and the user_id in u32 cannot be converted into i32 while fetching a contest");

        joinable!(schema::contest_users -> schema::contests (contest_id));
        schema::contest_users::dsl::contest_users
            .inner_join(
                schema::contests::table.on(schema::contest_users::contest_id
                    .nullable()
                    .eq(schema::contests::contest_id.nullable())),
            )
            .filter(schema::contest_users::user_id.eq(user_id))
            .select(ALL_COLUMNS)
    }

    pub fn find(
        contest_id: i32,
    ) -> diesel::dsl::Find<diesel::dsl::Select<schema::contests::table, AllColumns>, i32> {
        Self::select().find(contest_id)
    }

    pub fn find_registered(
        user_id: u32,
        contest_id: i32,
    ) -> diesel::dsl::Filter<
        diesel::dsl::Select<
            diesel::dsl::InnerJoin<schema::contest_users::table, schema::contests::table>,
            AllColumns,
        >,
        diesel::expression::operators::And<
            diesel::expression::operators::Eq<
                schema::contest_users::columns::user_id,
                diesel::expression::bound::Bound<diesel::sql_types::Integer, i32>,
            >,
            diesel::expression::operators::Eq<
                schema::contests::columns::contest_id,
                diesel::expression::bound::Bound<diesel::sql_types::Integer, i32>,
            >,
        >,
    > {
        Self::select_registered(user_id).filter(schema::contests::contest_id.eq(contest_id))
    }
}
