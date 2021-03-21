use tokio_diesel::{AsyncRunQueryDsl, OptionalExtension};

#[derive(Debug, serde::Deserialize)]
pub struct GetUserDetailsRequest {
    auth: crate::sessions::resolvers::AuthRequiredRequest,
    user_id: u32,
}

pub struct UserDetails {
    pub auth: crate::sessions::resolvers::AdminAuth,
    pub user: super::models::User,
}

impl UserDetails {
    pub async fn resolve(
        request: GetUserDetailsRequest,
        pool: &crate::helpers::DbPool,
    ) -> Result<Self, super::errors::GetUserDetailsError> {
        let auth = crate::sessions::resolvers::AdminAuth::resolve(request.auth, &pool).await?;
        let user_id = request.user_id;
        let user = super::models::User::find(user_id)
            .first_async::<super::models::User>(&pool)
            .await
            .optional()?
            .ok_or_else(|| super::errors::GetUserDetailsError::UnknownUser { user_id })?;
        Ok(Self { auth, user })
    }
}

#[derive(Debug, serde::Serialize)]
pub struct GetUserDetailsResponse {
    user_id: u32,
    email: String,
    nickname: String,
    full_name: String,
    access: u32,
    created: i32,
    lastlogin: i32,
    options: u32,
    messages: i32,
    avatar: String,
    city_name: String,
    region_name: String,
    country_name: String,
    job: String,
    is_activated: i8,
}

impl From<UserDetails> for GetUserDetailsResponse {
    fn from(user_details: UserDetails) -> Self {
        let user = user_details.user;
        Self {
            user_id: user.user_id,
            email: user.email,
            nickname: user.nickname,
            full_name: user.full_name,
            access: user.access,
            created: user.created,
            lastlogin: user.lastlogin,
            options: user.options,
            messages: user.messages,
            avatar: user.avatar,
            city_name: user.city_name,
            region_name: user.region_name,
            country_name: user.country_name,
            job: user.job,
            is_activated: user.is_activated,
        }
    }
}
