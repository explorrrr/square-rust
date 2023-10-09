//! Represents an error encountered during a request to the Connect API.

use crate::api::models::error::SquareError;

/// Represents an error encountered during a request to the Connect API.
#[derive(Clone, Debug, Default)]
pub struct SquareApiError {
    pub message: String,
    pub errors: Vec<SquareError>,
}

impl SquareApiError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_owned(),
            ..Default::default()
        }
    }

    /// Create a new SquareApiError with a message and a list of SquareErrors
    pub fn with_response_errors(message: &str, errors: &[SquareError]) -> Self {
        Self {
            message: message.to_owned(),
            errors: errors.to_vec(),
        }
    }
}

impl std::fmt::Display for SquareApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "API Error: {:?}", self)
    }
}

impl std::error::Error for SquareApiError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
