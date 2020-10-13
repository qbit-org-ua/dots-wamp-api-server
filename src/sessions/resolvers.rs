use diesel::r2d2::{ConnectionManager, Pool};
use tokio_diesel::AsyncRunQueryDsl;

#[derive(Debug, serde::Deserialize)]
pub struct AuthRequiredRequest {
    session_id: String,
}

struct AuthResolver {
    pub session: super::models::Session,
    pub user: Option<crate::users::models::User>,
}

impl AuthResolver {
    pub async fn resolve(
        AuthRequiredRequest { session_id }: AuthRequiredRequest,
        pool: &std::sync::Arc<Pool<ConnectionManager<diesel::MysqlConnection>>>,
    ) -> Result<Self, ()> {
        let session = super::models::Session::find(session_id)
            .first_async::<super::models::Session>(&pool)
            .await
            .map_err(|_| ())?;
        let user = crate::users::models::User::find(session.user_id().ok_or_else(|| ())?)
            .first_async(&pool)
            .await
            .ok();
        Ok(Self { session, user })
    }
}

pub struct AuthRequiredResolver {
    pub session: super::models::Session,
    pub user: crate::users::models::User,
}

impl AuthRequiredResolver {
    pub async fn resolve(
        request: AuthRequiredRequest,
        pool: &std::sync::Arc<Pool<ConnectionManager<diesel::MysqlConnection>>>,
    ) -> Result<Self, ()> {
        let resolved = AuthResolver::resolve(request, pool).await?;
        if let Some(user) = resolved.user {
            Ok(Self {
                session: resolved.session,
                user,
            })
        } else {
            Err(())
        }
    }
}

pub struct AdminAuthRequiredResolver(AuthRequiredResolver);

impl From<AdminAuthRequiredResolver> for AuthRequiredResolver {
    fn from(admin_resolver: AdminAuthRequiredResolver) -> Self {
        admin_resolver.0
    }
}

impl AdminAuthRequiredResolver {
    pub async fn resolve(
        request: AuthRequiredRequest,
        pool: &std::sync::Arc<Pool<ConnectionManager<diesel::MysqlConnection>>>,
    ) -> Result<Self, ()> {
        let resolved = AuthRequiredResolver::resolve(request, pool).await?;
        if resolved.user.access >= 65000 {
            Ok(Self(resolved))
        } else {
            Err(())
        }
    }
}
