#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("Unknown session with ID {session_id}")]
    UnknownSession { session_id: String },
    #[error("Unknown user with ID {user_id}")]
    UnknownUser { user_id: u32 },
    #[error("Permission denied, you need to have at least {expected_permission} permission")]
    PermissionDenied { expected_permission: String },
}
