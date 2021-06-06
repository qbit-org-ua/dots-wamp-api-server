use tokio_diesel::{AsyncRunQueryDsl, OptionalExtension};

#[derive(Debug, serde::Deserialize)]
pub struct GetProblemDetailsRequest {
    auth: crate::sessions::resolvers::AuthRequiredRequest,
    problem_id: u32,
}

pub struct ProblemDetails {
    pub auth: crate::sessions::resolvers::Auth,
    pub problem: super::models::Problem,
}

impl ProblemDetails {
    pub async fn resolve(
        GetProblemDetailsRequest { auth, problem_id }: GetProblemDetailsRequest,
        pool: &crate::helpers::DbPool,
    ) -> Result<Self, super::errors::GetProblemDetailsError> {
        let auth = crate::sessions::resolvers::Auth::resolve(auth, &pool).await?;
        let problem = super::models::Problem::find(problem_id)
            .first_async::<super::models::Problem>(&pool)
            .await
            .optional()?
            .ok_or_else(|| super::errors::GetProblemDetailsError::UnknownProblem {
                problem_id,
            })?;
        if !auth.user.is_admin() && auth.user.user_id != problem.user_id {
            return Err(super::errors::GetProblemDetailsError::Auth {
                error: crate::sessions::errors::AuthError::PermissionDenied {
                    expected_permission: "author".to_owned(),
                },
            });
        }
        Ok(Self { auth, problem })
    }
}

#[derive(Debug, serde::Serialize)]
pub struct GetProblemDetailsResponse {
    problem_id: u32,
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
    is_review_passed: bool,
}

impl From<super::models::Problem> for GetProblemDetailsResponse {
    fn from(problem: super::models::Problem) -> Self {
        Self {
            problem_id: problem.problem_id,
            problem_id: problem.problem_id,
            user_id: problem.user_id,
            contest_id: problem.contest_id,
            filename: problem.filename,
            checksum: problem.checksum,
            lang_id: problem.lang_id,
            check_type: problem.check_type,
            posted_time: problem.posted_time,
            checked_time: problem.checked_time,
            contest_time: problem.contest_time,
            test_result: problem.test_result,
            //test_score: problem.test_score,
            //score: problem.score,
            module_val: problem.module_val,
            compile_error: problem.compile_error,
            is_review_passed: problem.is_review_passed == 1,
        }
    }
}
