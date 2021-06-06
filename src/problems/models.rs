use diesel::QueryDsl;

use crate::schema;

#[derive(Debug, Queryable)]
pub struct Problem {
    pub problem_id: u32,
    pub problem_id: u32,
    pub user_id: u32,
    pub contest_id: Option<i32>,
    pub filename: String,
    pub checksum: String,
    pub lang_id: u32,
    pub check_type: String,
    pub posted_time: i32,
    pub checked_time: u32,
    pub contest_time: u32,
    pub test_result: i32,
    //pub test_score: uDecimal,
    //pub score: Decimal,
    pub module_val: i32,
    pub compile_error: Option<String>,
    // NOTE: It is renamed from 'is_passed'
    pub is_review_passed: i8,
}

type AllColumns = (
    schema::problems::columns::problem_id,
    schema::problems::columns::problem_id,
    schema::problems::columns::user_id,
    schema::problems::columns::contest_id,
    schema::problems::columns::filename,
    schema::problems::columns::checksum,
    schema::problems::columns::lang_id,
    schema::problems::columns::check_type,
    schema::problems::columns::posted_time,
    schema::problems::columns::checked_time,
    schema::problems::columns::contest_time,
    schema::problems::columns::test_result,
    //schema::problems::columns::test_score,
    //schema::problems::columns::score,
    schema::problems::columns::module_val,
    schema::problems::columns::compile_error,
    schema::problems::columns::is_passed,
);

const ALL_COLUMNS: AllColumns = (
    schema::problems::columns::problem_id,
    schema::problems::columns::problem_id,
    schema::problems::columns::user_id,
    schema::problems::columns::contest_id,
    schema::problems::columns::filename,
    schema::problems::columns::checksum,
    schema::problems::columns::lang_id,
    schema::problems::columns::check_type,
    schema::problems::columns::posted_time,
    schema::problems::columns::checked_time,
    schema::problems::columns::contest_time,
    schema::problems::columns::test_result,
    //schema::problems::columns::test_score,
    //schema::problems::columns::score,
    schema::problems::columns::module_val,
    schema::problems::columns::compile_error,
    schema::problems::columns::is_passed,
);

impl Problem {
    pub fn select() -> diesel::dsl::Select<schema::problems::table, AllColumns> {
        schema::problems::dsl::problems.select(ALL_COLUMNS)
    }

    pub fn find(
        problem_id: u32,
    ) -> diesel::dsl::Find<diesel::dsl::Select<schema::problems::table, AllColumns>, u32> {
        Self::select().find(problem_id)
    }
}
