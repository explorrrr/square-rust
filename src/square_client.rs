//! The SquareApiClient is the main entry point for making requests to the Square API

use crate::api::customers::CustomersApi;
use crate::api::models::api_error::SquareApiError;
use crate::config::SquareApiConfig;
use crate::http::client::http_client::SquareHttpClient;

/// The SquareApiClient is the main entry point for making requests to the Square API
pub struct SquareApiClient {
    pub customers: CustomersApi,
}

impl SquareApiClient {
    /// Create a new SquareApiClient
    /// # Example - Create a new SquareApiClient with Sandbox environment and default http client
    /// ```
    /// use square_rust::square_client::SquareApiClient;
    /// use square_rust::config::SquareApiConfig;
    /// use square_rust::environment::Environment;
    ///
    /// let config = SquareApiConfig::builder()
    ///    .environment(Environment::Sandbox)
    ///    .access_token("access_token".to_string())
    ///    .build();
    /// ```
    ///
    /// # Example - Create a new SquareApiClient with Production environment and default http client
    /// ```no_run
    /// use square_rust::square_client::SquareApiClient;
    /// use square_rust::config::SquareApiConfig;
    /// use square_rust::environment::Environment;
    ///
    /// let config = SquareApiConfig::builder()
    ///   .environment(Environment::Production)
    ///   .access_token("access_token".to_string())
    ///   .build();
    /// ```
    pub fn try_new(config: Option<SquareApiConfig>) -> Result<Self, SquareApiError> {
        let config = config.unwrap_or_else(|| SquareApiConfig::builder().build());
        let http_client = SquareHttpClient::try_new(&config.http_client_config)?;
        Ok(SquareApiClient {
            customers: CustomersApi::new(config.clone(), http_client.clone()),
        })
    }
}
