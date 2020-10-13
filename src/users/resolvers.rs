use diesel::r2d2::{ConnectionManager, Pool};
use tokio_diesel::AsyncRunQueryDsl;

#[derive(Debug, serde::Deserialize)]
pub struct UserDetailsRequest {
    auth: crate::sessions::resolvers::AuthRequiredRequest,
    user_id: u32,
}

pub struct UserDetailsRequestResolver {
    pub auth: crate::sessions::resolvers::AdminAuthRequiredResolver,
    pub user: super::models::User,
}

impl UserDetailsRequestResolver {
    pub async fn resolve(
        UserDetailsRequest { auth, user_id }: UserDetailsRequest,
        pool: &std::sync::Arc<Pool<ConnectionManager<diesel::MysqlConnection>>>,
    ) -> Result<Self, ()> {
        let auth =
            crate::sessions::resolvers::AdminAuthRequiredResolver::resolve(auth, &pool).await?;
        let user = super::models::User::find(user_id)
            .first_async::<super::models::User>(&pool)
            .await
            .map_err(|_| ())?;
        Ok(Self { auth, user })
    }
}

#[derive(Debug, serde::Serialize)]
pub struct UserDetailsResponse {
    user_id: u32,
    email: String,
    //password: String,
    nickname: String,
    //birthday: Date,
    access: u32,
    created: i32,
    lastlogin: i32,
    options: u32,
    messages: i32,
    avatar: String,
    city_name: String,
    region_name: String,
    country_name: String,
    full_name: String,
    job: String,
    is_activated: i8,
}

impl From<super::models::User> for UserDetailsResponse {
    fn from(user: super::models::User) -> Self {
        Self {
            user_id: user.user_id,
            email: user.email,
            nickname: user.nickname,
            //birthday:
            access: user.access,
            created: user.created,
            lastlogin: user.lastlogin,
            options: user.options,
            messages: user.messages,
            avatar: user.avatar,
            city_name: user.city_name,
            region_name: user.region_name,
            country_name: user.country_name,
            full_name: user.FIO,
            job: user.job,
            is_activated: user.is_activated,
        }
    }
}
