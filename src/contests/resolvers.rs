use tokio_diesel::{AsyncRunQueryDsl, OptionalExtension};

#[derive(Debug, serde::Deserialize)]
pub struct GetContestDetailsRequest {
    auth: crate::sessions::resolvers::AuthRequiredRequest,
    contest_id: i32,
}

pub struct ContestDetails {
    pub auth: crate::sessions::resolvers::Auth,
    pub contest: super::models::Contest,
}

impl ContestDetails {
    pub async fn resolve(
        GetContestDetailsRequest { auth, contest_id }: GetContestDetailsRequest,
        pool: &crate::helpers::DbPool,
    ) -> Result<Self, super::errors::GetContestDetailsError> {
        let auth = crate::sessions::resolvers::Auth::resolve(auth, &pool).await?;
        let contest = if auth.user.is_admin() {
            super::models::Contest::find(contest_id)
                .first_async::<super::models::Contest>(&pool)
                .await
                .optional()?
                .ok_or_else(|| super::errors::GetContestDetailsError::Auth {
                    error: crate::sessions::errors::AuthError::PermissionDenied {
                        expected_permission: "registered".to_owned(),
                    },
                })?
        } else {
            super::models::Contest::find_registered(auth.user.user_id, contest_id)
                .first_async::<super::models::Contest>(&pool)
                .await
                .optional()?
                .ok_or_else(|| super::errors::GetContestDetailsError::UnknownContest {
                    contest_id,
                })?
        };
        Ok(Self { auth, contest })
    }
}

#[derive(Debug, serde::Serialize)]
pub struct GetContestDetailsResponse {
    contest_id: i32,
    title: String,
    contest_type: String,
    start_time: i64,
    options: i32,
    data: String,
    info: String,
    visible: bool,
    author_id: i32,
    allow_languages: String,
}

impl From<super::models::Contest> for GetContestDetailsResponse {
    fn from(contest: super::models::Contest) -> Self {
        Self {
            contest_id: contest.contest_id,
            title: contest.title,
            contest_type: contest.contest_type,
            start_time: contest.start_time,
            options: contest.options,
            data: contest.data,
            info: contest.info,
            visible: contest.visible,
            author_id: contest.author_id,
            allow_languages: contest.allow_languages,
        }
    }
}
