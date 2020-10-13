use diesel::r2d2::{ConnectionManager, Pool};
use tokio_diesel::AsyncRunQueryDsl;

#[derive(Debug, serde::Deserialize)]
pub struct SolutionDetailsRequest {
    auth: crate::sessions::resolvers::AuthRequiredRequest,
    solution_id: u32,
}

pub struct SolutionDetailsRequestResolver {
    pub auth: crate::sessions::resolvers::AuthRequiredResolver,
    pub solution: super::models::Solution,
}

impl SolutionDetailsRequestResolver {
    pub async fn resolve(
        SolutionDetailsRequest { auth, solution_id }: SolutionDetailsRequest,
        pool: &std::sync::Arc<Pool<ConnectionManager<diesel::MysqlConnection>>>,
    ) -> Result<Self, ()> {
        let auth = crate::sessions::resolvers::AuthRequiredResolver::resolve(auth, &pool).await?;
        let solution = super::models::Solution::find(solution_id)
            .first_async::<super::models::Solution>(&pool)
            .await
            .map_err(|_| ())?;
        if !auth.user.is_admin() && auth.user.user_id != solution.user_id {
            return Err(());
        }
        Ok(Self { auth, solution })
    }
}

#[derive(Debug, serde::Serialize)]
pub struct SolutionDetailsResponse {
    solution_id: u32,
    problem_id: u32,
    user_id: u32,
    contest_id: Option<i32>,
    filename: String,
    checksum: String,
    lang_id: u32,
    check_type: String,
    posted_time: i32,
    checked_time: u32,
    contest_time: u32,
    test_result: i32,
    //test_score: uDecimal,
    //score: Decimal,
    module_val: i32,
    compile_error: Option<String>,
    is_passed: i8,
}

impl From<super::models::Solution> for SolutionDetailsResponse {
    fn from(solution: super::models::Solution) -> Self {
        Self {
            solution_id: solution.solution_id,
            problem_id: solution.problem_id,
            user_id: solution.user_id,
            contest_id: solution.contest_id,
            filename: solution.filename,
            checksum: solution.checksum,
            lang_id: solution.lang_id,
            check_type: solution.check_type,
            posted_time: solution.posted_time,
            checked_time: solution.checked_time,
            contest_time: solution.contest_time,
            test_result: solution.test_result,
            //test_score: solution.test_score,
            //score: solution.score,
            module_val: solution.module_val,
            compile_error: solution.compile_error,
            is_passed: solution.is_passed,
        }
    }
}
