//! Configuration for the Square API

use std::env;

use log::warn;

use crate::api::models::enums::api_version::SquareApiVersion;
use crate::environment::Environment;
use crate::http::client::config::SquareHttpClientConfig;

const DEFAULT_BASE_URL: &str = "/v2";
pub(crate) const DEFAULT_API_VERSION: SquareApiVersion = SquareApiVersion::V20230925;

/// A builder struct to construct a SquareApiConfig
#[derive(Default)]
pub struct SquareApiConfigBuilder {
    environment: Option<Environment>,
    api_version: Option<SquareApiVersion>,
    http_client_config: Option<SquareHttpClientConfig>,
    access_token: Option<String>,
    base_url: Option<String>,
}

/// A builder to construct a SquareApiConfig
impl SquareApiConfigBuilder {
    /// Set the environment
    pub fn environment(&mut self, environment: Environment) -> &mut Self {
        self.environment = Some(environment);
        self
    }

    /// Set the api version
    pub fn api_version(&mut self, api_version: SquareApiVersion) -> &mut Self {
        self.api_version = Some(api_version);
        self
    }

    /// Set the http client config
    pub fn http_client_config(&mut self, http_client_config: SquareHttpClientConfig) -> &mut Self {
        self.http_client_config = Some(http_client_config);
        self
    }

    /// Set the access token
    pub fn access_token(&mut self, access_token: String) -> &mut Self {
        self.access_token = Some(access_token);
        self
    }

    /// Set the base url
    pub fn base_url(&mut self, base_url: String) -> &mut Self {
        self.base_url = Some(base_url);
        self
    }

    /// Build the SquareApiConfig
    pub fn build(&self) -> SquareApiConfig {
        SquareApiConfig {
            environment: self.environment.clone().unwrap_or_default(),
            api_version: self.api_version.clone().unwrap_or(DEFAULT_API_VERSION),
            http_client_config: self.http_client_config.clone().unwrap_or_default(),
            access_token: self
                .access_token
                .clone()
                .unwrap_or_else(get_default_authorization_token),
            base_url: self.base_url.clone().unwrap_or_else(|| DEFAULT_BASE_URL.to_owned()),
        }
    }
}

/// A struct to hold the Square API configuration
#[derive(Clone)]
pub struct SquareApiConfig {
    pub environment: Environment,
    pub api_version: SquareApiVersion,
    pub http_client_config: SquareHttpClientConfig,
    pub access_token: String,
    pub base_url: String,
}

impl SquareApiConfig {
    /// Create a new SquareApiConfigBuilder
    pub fn builder() -> SquareApiConfigBuilder {
        SquareApiConfigBuilder::default()
    }

    /// Get DNS from given config
    pub fn get_dns(&self) -> String {
        match self.environment {
            Environment::Mock => "mock".to_string(),
            Environment::Sandbox => "https://connect.squareupsandbox.com".to_string(),
            Environment::Production => "https://connect.squareup.com".to_string(),
        }
    }
}

/// Get the default authorization token from the environment variable SQUARE_API_ACCESS_TOKEN
pub(crate) fn get_default_authorization_token() -> String {
    format!(
        "Bearer {}",
        env::var("SQUARE_API_ACCESS_TOKEN").unwrap_or_else(|_| {
            warn!("Environment variable SQUARE_API_ACCESS_TOKEN not found");
            String::new()
        })
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = SquareApiConfig::builder().build();
        assert_eq!(config.environment, Environment::Sandbox);
        assert_eq!(config.base_url, DEFAULT_BASE_URL);
    }

    #[test]
    fn test_get_dns() {
        let config = SquareApiConfig::builder().build();
        let dns = config.get_dns();
        assert_eq!(dns, "https://connect.squareupsandbox.com");
    }
}
