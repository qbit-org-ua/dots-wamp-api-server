#[derive(Debug, thiserror::Error)]
pub enum GetUserDetailsError {
    #[error("Internal error: {error_message}")]
    InternalError { error_message: String },
    #[error("Authentication error: {error}")]
    Auth {
        #[from]
        error: crate::sessions::errors::AuthError,
    },
    #[error("User with ID #{user_id} is unknown")]
    UnknownUser { user_id: u32 },
}

impl From<tokio_diesel::AsyncError> for GetUserDetailsError {
    fn from(err: tokio_diesel::AsyncError) -> Self {
        Self::InternalError {
            error_message: err.to_string(),
        }
    }
}

impl From<GetUserDetailsError> for wamp_async::WampError {
    fn from(err: GetUserDetailsError) -> Self {
        Self::UnknownError(format!("Failed to get user details: {}", err))
    }
}
