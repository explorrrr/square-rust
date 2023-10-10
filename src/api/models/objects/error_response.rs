//! Error response

use serde::Deserialize;

use crate::api::models::objects::error::SquareError;

/// A generic error response from the Square API.
#[derive(Clone, Deserialize)]
pub struct ErrorResponse {
    /// The list of errors.
    pub errors: Vec<SquareError>,
}
