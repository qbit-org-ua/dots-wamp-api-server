#[derive(Debug, thiserror::Error)]
pub enum GetSolutionDetailsError {
    #[error("Internal error: {error_message}")]
    InternalError { error_message: String },
    #[error("Authentication error: {error}")]
    Auth {
        #[from]
        error: crate::sessions::errors::AuthError,
    },
    #[error("Solution with ID #{solution_id} is unknown")]
    UnknownSolution { solution_id: u32 },
    #[error("Permission denied due to: {error_message}")]
    PermissionDenied { error_message: String },
}

impl From<tokio_diesel::AsyncError> for GetSolutionDetailsError {
    fn from(err: tokio_diesel::AsyncError) -> Self {
        Self::InternalError {
            error_message: err.to_string(),
        }
    }
}
