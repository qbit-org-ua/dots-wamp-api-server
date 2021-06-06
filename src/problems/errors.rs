#[derive(Debug, thiserror::Error)]
pub enum GetProblemDetailsError {
    #[error("Internal error: {error_message}")]
    InternalError { error_message: String },
    #[error("Authentication error: {error}")]
    Auth {
        #[from]
        error: crate::sessions::errors::AuthError,
    },
    #[error("Problem with ID #{problem_id} is unknown")]
    UnknownProblem { problem_id: u32 },
}

impl From<tokio_diesel::AsyncError> for GetProblemDetailsError {
    fn from(err: tokio_diesel::AsyncError) -> Self {
        Self::InternalError {
            error_message: err.to_string(),
        }
    }
}

impl From<GetProblemDetailsError> for wamp_async::WampError {
    fn from(err: GetProblemDetailsError) -> Self {
        Self::UnknownError(format!("Failed to get problem details: {}", err))
    }
}
