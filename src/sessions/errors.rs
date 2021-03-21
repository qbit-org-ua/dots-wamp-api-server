#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("Internal error: {error_message}")]
    InternalError { error_message: String },
    #[error("Unknown session with ID {session_id}")]
    UnknownSession { session_id: String },
    #[error("Unknown user with ID {user_id}")]
    UnknownUser { user_id: u32 },
    #[error("Permission denied, you need to have at least {expected_permission} permission")]
    PermissionDenied { expected_permission: String },
}

impl From<tokio_diesel::AsyncError> for AuthError {
    fn from(err: tokio_diesel::AsyncError) -> Self {
        Self::InternalError {
            error_message: err.to_string(),
        }
    }
}
