use diesel::QueryDsl;

use crate::schema;

#[derive(Debug, Queryable)]
pub struct Solution {
    pub solution_id: u32,
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
    pub is_passed: i8,
}

type AllColumns = (
    schema::solutions::columns::solution_id,
    schema::solutions::columns::problem_id,
    schema::solutions::columns::user_id,
    schema::solutions::columns::contest_id,
    schema::solutions::columns::filename,
    schema::solutions::columns::checksum,
    schema::solutions::columns::lang_id,
    schema::solutions::columns::check_type,
    schema::solutions::columns::posted_time,
    schema::solutions::columns::checked_time,
    schema::solutions::columns::contest_time,
    schema::solutions::columns::test_result,
    //schema::solutions::columns::test_score,
    //schema::solutions::columns::score,
    schema::solutions::columns::module_val,
    schema::solutions::columns::compile_error,
    schema::solutions::columns::is_passed,
);

const ALL_COLUMNS: AllColumns = (
    schema::solutions::columns::solution_id,
    schema::solutions::columns::problem_id,
    schema::solutions::columns::user_id,
    schema::solutions::columns::contest_id,
    schema::solutions::columns::filename,
    schema::solutions::columns::checksum,
    schema::solutions::columns::lang_id,
    schema::solutions::columns::check_type,
    schema::solutions::columns::posted_time,
    schema::solutions::columns::checked_time,
    schema::solutions::columns::contest_time,
    schema::solutions::columns::test_result,
    //schema::solutions::columns::test_score,
    //schema::solutions::columns::score,
    schema::solutions::columns::module_val,
    schema::solutions::columns::compile_error,
    schema::solutions::columns::is_passed,
);

impl Solution {
    pub fn select() -> diesel::dsl::Select<schema::solutions::table, AllColumns> {
        schema::solutions::dsl::solutions.select(ALL_COLUMNS)
    }

    pub fn find(
        solution_id: u32,
    ) -> diesel::dsl::Find<diesel::dsl::Select<schema::solutions::table, AllColumns>, u32> {
        Self::select().find(solution_id)
    }
}
