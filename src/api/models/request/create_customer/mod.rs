//! This module contains all version of request models for the create_customer endpoint.

pub mod versions;

use std::any::Any;
use crate::api::models::request::create_customer::versions::v20230925::CreateCustomerRequestV20230925;

/// CreateCustomerRequest
pub trait CreateCustomerRequest {
    fn as_any(&self) -> &dyn std::any::Any;
}

impl CreateCustomerRequest for CreateCustomerRequestV20230925 {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
