use std::convert::TryInto;

use chrono::TimeZone;
use diesel::{ExpressionMethods, JoinOnDsl, NullableExpressionMethods, QueryDsl};

use crate::schema;

#[derive(Debug, Queryable)]
pub struct ContestProblems {
    pub contest_id: i32,
    pub short_name: String,
    pub problem_id: i32,
    pub max_score: i32,
    pub is_with_code_review: bool,
    pub user_id: i32,
}

type AllColumns = (
    schema::contests::columns::contest_id,
    schema::contests::columns::short_name,
    schema::contests::columns::problem_id,
    schema::contests::columns::max_score,
    schema::contests::columns::is_with_code_review,
    schema::contests::columns::user_id,
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

impl ContestProblems {
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

    pub fn is_open_for_submition(&self) -> bool {
        chrono::Utc.timestamp(self.start_time, 0) > chrono::Utc::now()
    }
}
