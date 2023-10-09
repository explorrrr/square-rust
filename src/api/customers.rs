//! CustomerApi module

use super::models::response::create_customer::from_response_to_create_customer_response;
use crate::api::models::api_error::SquareApiError;
use crate::api::models::api_version::SquareApiVersion;
use crate::api::models::request::create_customer::versions::v20230925::CreateCustomerRequestV20230925;
use crate::api::models::request::create_customer::CreateCustomerRequest;
use crate::api::models::response::create_customer::CreateCustomerResponse;
use crate::config::SquareApiConfig;
use crate::http::client::http_client::SquareHttpClient;

const DEFAULT_API_URL: &str = "/customers";

/// CustomerApi module
pub struct CustomersApi {
    config: SquareApiConfig,
    client: SquareHttpClient,
}

impl CustomersApi {
    /// Create a new CustomersApi
    pub fn new(config: SquareApiConfig, client: SquareHttpClient) -> Self {
        CustomersApi { config, client }
    }

    /// Use a create_customer endpoint
    pub async fn create_customer<R: CreateCustomerRequest>(
        &self,
        request: R,
    ) -> Result<Box<dyn CreateCustomerResponse>, SquareApiError> {
        self.process_request(request).await
    }

    /// Process the request and deserialize the response
    async fn process_request<R: CreateCustomerRequest>(
        &self,
        request: R,
    ) -> Result<Box<dyn CreateCustomerResponse>, SquareApiError> {
        let body = match self.config.api_version {
            SquareApiVersion::V20230925 => {
                let data = request.as_any().downcast_ref::<CreateCustomerRequestV20230925>();
                match data {
                    Some(data) => data.to_body_string(),
                    None => return Err(SquareApiError::new("Error casting CreateCustomerRequestV20230925")),
                }
            }
        };

        // request to square api
        let response = self.client.post(&self.url(), &body).await?;
        let data = from_response_to_create_customer_response(response, self.config.api_version.clone()).await;
        data
    }

    /// Get the url for the request
    fn url(&self) -> String {
        format!("{}{}{}", self.config.get_dns(), self.config.base_url, DEFAULT_API_URL)
    }
}

#[cfg(test)]
mod tests {
    use tokio;

    use super::*;
    use crate::api::models::request::create_customer::versions::v20230925::CreateCustomerRequestV20230925;

    #[tokio::test]
    async fn test_create_customer() {
        let idempotency_key = None;
        let given_name = Some("given_name".to_string());
        let family_name = Some("family_name".to_string());
        let company_name = None;
        let nickname = None;
        let email_address = None;
        let address = None;
        let phone_number = None;
        let reference_id = None;
        let note = None;
        let birthday = None;
        let tax_ids = None;

        let config = SquareApiConfig::builder().build();
        let http_client = SquareHttpClient::try_new(&config.http_client_config).unwrap();
        let client = CustomersApi::new(config, http_client);
        let request = CreateCustomerRequestV20230925::new(
            idempotency_key,
            given_name,
            family_name,
            company_name,
            nickname,
            email_address,
            address,
            phone_number,
            reference_id,
            note,
            birthday,
            tax_ids,
        );
        let _ = client.create_customer(request).await.unwrap();
    }
}
