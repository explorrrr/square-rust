//! Models for errors returned by the Connect API.

use serde::{Deserialize, Serialize};
use crate::api::models::enums::error_category::ErrorCategory;
use crate::api::models::enums::error_code::ErrorCode;

/// Represents an error encountered during a request to the Connect API.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SquareError {
    /// The high-level category for the error.
    pub category: ErrorCategory,
    /// The specific code of the error.
    pub code: ErrorCode,
    /// A human-readable description of the error for debugging purposes.
    pub detail: Option<String>,
    /// The name of the field provided in the original request (if any) that the error pertains to.
    pub fileld: Option<String>,
}
