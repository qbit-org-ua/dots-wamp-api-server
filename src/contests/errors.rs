#[derive(Debug, thiserror::Error)]
pub enum GetContestDetailsError {
    #[error("Internal error: {error_message}")]
    InternalError { error_message: String },
    #[error("Authentication error: {error}")]
    Auth {
        #[from]
        error: crate::sessions::errors::AuthError,
    },
    #[error("Contest with ID #{contest_id} is unknown")]
    UnknownContest { contest_id: i32 },
}

impl From<tokio_diesel::AsyncError> for GetContestDetailsError {
    fn from(err: tokio_diesel::AsyncError) -> Self {
        Self::InternalError {
            error_message: err.to_string(),
        }
    }
}

impl From<GetContestDetailsError> for wamp_async::WampError {
    fn from(err: GetContestDetailsError) -> Self {
        Self::UnknownError(format!("Failed to get contest details: {}", err))
    }
}
