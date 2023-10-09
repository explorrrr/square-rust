//! This module contains all version of response models for the create_customer endpoint.

pub mod versions;

use std::any::Any;
use crate::{api::models::{response::create_customer::versions::v20230925::CreateCustomerResponseV20230925, api_error::SquareApiError, error_response::ErrorResponse}};
use crate::api::models::api_version::SquareApiVersion;
use reqwest::Response;
use reqwest::Error;

/// This trait represents the response from the create_customer endpoint
pub trait CreateCustomerResponse {
    fn as_any(&self) -> &dyn std::any::Any;
}

impl CreateCustomerResponse for CreateCustomerResponseV20230925 {
    fn as_any(&self) -> &dyn Any {
        self
    }

}

/// Convert the response from the create_customer endpoint into the appropriate response type
pub async fn from_response_to_create_customer_response(response: Response, api_version: SquareApiVersion) -> Result<Box<dyn CreateCustomerResponse>, SquareApiError> {
    if response.status().is_success() {
        let parsed = match api_version {
            SquareApiVersion::V20230925 => {
                let parsed: Result<CreateCustomerResponseV20230925, Error> = response.json::<CreateCustomerResponseV20230925>().await;
                parsed
            }
        };

        match parsed {
            Ok(val) => Ok(Box::new(val) as Box<dyn CreateCustomerResponse>),
            Err(_) => Err(SquareApiError::new("Deserialization error")),
        }
    } else {
        let err_response_res: Result<ErrorResponse, Error> = response.json().await;
        match err_response_res {
            Ok(error_response) => {
                let api_error = SquareApiError::with_response_errors("Error response", &error_response.errors);
                Err(api_error)
            },
            Err(_) => Err(SquareApiError::new("Deserialization error")),
        }
    }
}
