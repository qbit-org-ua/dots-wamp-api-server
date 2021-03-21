use tokio_diesel::{AsyncRunQueryDsl, OptionalExtension};

#[derive(Debug, serde::Deserialize)]
pub struct AuthRequiredRequest {
    session_id: String,
}

pub struct Auth {
    pub session: super::models::Session,
    pub user: crate::users::models::User,
}

impl Auth {
    pub async fn resolve(
        AuthRequiredRequest { session_id }: AuthRequiredRequest,
        pool: &crate::helpers::DbPool,
    ) -> Result<Self, super::errors::AuthError> {
        let session = super::models::Session::find(session_id.clone())
            .first_async::<super::models::Session>(&pool)
            .await
            .optional()?
            .ok_or_else(|| super::errors::AuthError::UnknownSession {
                session_id: session_id.clone(),
            })?;
        let user_id = session
            .user_id()
            .ok_or_else(|| super::errors::AuthError::UnknownSession { session_id })?;
        let user = crate::users::models::User::find(user_id)
            .first_async(&pool)
            .await
            .map_err(|_| super::errors::AuthError::UnknownUser { user_id })?;
        Ok(Self { session, user })
    }
}

pub struct AdminAuth(Auth);

impl From<AdminAuth> for Auth {
    fn from(admin_resolver: AdminAuth) -> Self {
        admin_resolver.0
    }
}

impl AdminAuth {
    pub async fn resolve(
        request: AuthRequiredRequest,
        pool: &crate::helpers::DbPool,
    ) -> Result<Self, super::errors::AuthError> {
        let resolved = Auth::resolve(request, pool).await?;
        if resolved.user.is_admin() {
            Ok(Self(resolved))
        } else {
            Err(super::errors::AuthError::PermissionDenied {
                expected_permission: "admin".to_owned(),
            })
        }
    }
}
