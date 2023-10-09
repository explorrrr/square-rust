//! Models for responses from the create_customer endpoint.

use serde::{Deserialize, Serialize};

use crate::api::models::customer::Customer;
use crate::api::models::error::SquareError;

/// CreateCustomerResponse for 2023-09-25 api version
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCustomerResponseV20230925 {
    errors: Option<Vec<SquareError>>,
    customer: Option<Customer>,
}
