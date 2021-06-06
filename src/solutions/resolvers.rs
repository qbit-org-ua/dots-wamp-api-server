use tokio_diesel::{AsyncRunQueryDsl, OptionalExtension};

#[derive(Debug, serde::Deserialize)]
pub struct GetSolutionDetailsRequest {
    auth: crate::sessions::resolvers::AuthRequiredRequest,
    solution_id: u32,
}

pub struct SolutionDetails {
    pub auth: crate::sessions::resolvers::Auth,
    pub solution: super::models::Solution,
}

impl SolutionDetails {
    pub async fn resolve(
        GetSolutionDetailsRequest { auth, solution_id }: GetSolutionDetailsRequest,
        pool: &crate::helpers::DbPool,
    ) -> Result<Self, super::errors::GetSolutionDetailsError> {
        let auth = crate::sessions::resolvers::Auth::resolve(auth, &pool).await?;
        let solution = super::models::Solution::find(solution_id)
            .first_async::<super::models::Solution>(&pool)
            .await
            .optional()?
            .ok_or_else(|| super::errors::GetSolutionDetailsError::UnknownSolution {
                solution_id,
            })?;
        if !auth.user.is_admin() && auth.user.user_id != solution.user_id {
            return Err(super::errors::GetSolutionDetailsError::Auth {
                error: crate::sessions::errors::AuthError::PermissionDenied {
                    expected_permission: "author".to_owned(),
                },
            });
        }
        Ok(Self { auth, solution })
    }
}

#[derive(Debug, serde::Serialize)]
pub struct GetSolutionDetailsResponse {
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
    is_review_passed: bool,
}

impl From<super::models::Solution> for GetSolutionDetailsResponse {
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
            is_review_passed: solution.is_review_passed == 1,
        }
    }
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RecentSolutionStatus {
    New,
    Queued,
    Executing,
    Tested,
}

impl RecentSolutionStatus {
    fn from_test_result(test_result: i32) -> Self {
        match test_result {
            -1 => Self::New,
            -2 => Self::Queued,
            -3 => Self::Executing,
            _ => Self::Tested,
        }
    }
}

#[derive(Debug, serde::Serialize)]
pub struct RecentSolution {
    solution_id: u32,
    updated_at_unixtime: u32,
    basic_status: RecentSolutionStatus,
}

impl From<super::models::Solution> for RecentSolution {
    fn from(solution: super::models::Solution) -> Self {
        Self {
            solution_id: solution.solution_id,
            updated_at_unixtime: solution.checked_time,
            basic_status: RecentSolutionStatus::from_test_result(solution.test_result),
        }
    }
}

#[derive(Debug, serde::Serialize)]
pub struct RecentSolutionEvents {
    recent_solutions: Vec<RecentSolution>,
}

impl From<Vec<super::models::Solution>> for RecentSolutionEvents {
    fn from(recent_solutions: Vec<super::models::Solution>) -> Self {
        Self {
            recent_solutions: recent_solutions
                .into_iter()
                .map(crate::solutions::resolvers::RecentSolution::from)
                .collect::<Vec<_>>(),
        }
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum SolutionExecutionEnvironment {
    /// C (legacy compiler)
    C = 2,
    /// C (2011)
    C11 = 18,

    /// C++ (2003)
    Cxx = 3,
    /// C++ (2011)
    Cxx11 = 19,
    /// C++ (2014)
    Cxx14 = 20,
    /// C++ (2017)
    Cxx17 = 30,

    /// Free Pascal
    Fpc = 4,
    /// Free Pascal with Delphi mode
    FpcDelphi = 39,

    // Other compiled languages
    Go = 16,
    Haskell = 21,
    Nim = 22,
    Rust = 23,
    Ocaml = 33,
    Swift = 34,

    // .NET
    CSharpDotnet = 35,
    CSharpMono = 14,
    VisualBasicMono = 29,

    // Java and Java-based languages
    JavaOpenJdk7 = 13,
    JavaOracleJdk8 = 17,
    Scala = 24,
    Kotlin = 26,

    // Python
    Python2 = 11,
    Python3 = 12,
    Python3WithMachineLearningPackages = 28,

    // Other interpreted languages
    Ruby = 15,
    Php = 25,
    Bash = 27,
    Javascript = 31,
}

#[derive(Debug, serde::Deserialize)]
pub struct SubmitSolutionRequest {
    auth: crate::sessions::resolvers::AuthRequiredRequest,
    contest_id: i32,
    problem_id: i32,
    execution_environment: SolutionExecutionEnvironment,
    solution: String,
}

pub struct SolutionSubmition {
    pub auth: crate::sessions::resolvers::Auth,
    pub contest: (),
    pub problem: (),
    pub solution: String,
}

impl SolutionSubmition {
    pub async fn resolve(
        SubmitSolutionRequest {
            auth,
            contest_id,
            problem_id,
            execution_environment,
            solution,
        }: SubmitSolutionRequest,
        pool: &crate::helpers::DbPool,
    ) -> Result<Self, super::errors::GetSolutionDetailsError> {
        let auth = crate::sessions::resolvers::Auth::resolve(auth, &pool).await?;
        crate::contests::resolvers::ContestProblem::resolve(auth, contest_id, problem_id).await?;
        let solution = super::models::Solution::find(solution_id)
            .first_async::<super::models::Solution>(&pool)
            .await
            .optional()?
            .ok_or_else(|| super::errors::GetSolutionDetailsError::UnknownSolution {
                solution_id,
            })?;
        if !auth.user.is_admin() && auth.user.user_id != solution.user_id {
            return Err(super::errors::GetSolutionDetailsError::Auth {
                error: crate::sessions::errors::AuthError::PermissionDenied {
                    expected_permission: "author".to_owned(),
                },
            });
        }
        Ok(Self {
            auth,
            contest,
            problem,
            solution,
        })
    }
}

#[derive(Debug, serde::Serialize)]
pub struct SubmitSolutionResponse {
    solution_id: u32,
    problem_id: u32,
    user_id: u32,
    contest_id: Option<i32>,
    checksum: String,
    lang_id: u32,
    check_type: String,
    posted_time: i32,
    contest_time: u32,
}

impl From<super::models::Solution> for SubmitSolutionResponse {
    fn from(solution: super::models::Solution) -> Self {
        Self {
            solution_id: solution.solution_id,
            problem_id: solution.problem_id,
            user_id: solution.user_id,
            contest_id: solution.contest_id,
            checksum: solution.checksum,
            lang_id: solution.lang_id,
            check_type: solution.check_type,
            posted_time: solution.posted_time,
            contest_time: solution.contest_time,
        }
    }
}
