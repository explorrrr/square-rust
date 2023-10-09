//! Configuration for HTTP client settings

use std::default::Default;
use std::env;
use std::time::Duration;

use crate::http::headers::Headers;

const DEFAULT_TIMEOUT: u32 = 60;

/// Configuration struct for HTTP client settings
#[derive(Clone, Debug)]
pub struct SquareHttpClientConfig {
    /// Timeout for HTTP connections
    pub timeout: u32,
    /// User Agent to use for requests
    pub user_agent: String,
    /// Headers to send with each request
    pub default_headers: Headers,
    /// Retry mechanism configuration
    pub retry_configuration: RetryConfig,
}

/// Retry mechanism configuration
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RetryConfig {
    /// How many times client should call same request in case of failure.
    pub retries_count: u32,
    /// Minimum waiting time between two retry attempts (it can end up being lower due to jittering).
    pub min_retry_interval: Duration,
    /// Maximum waiting time between two retry attempts.
    pub max_retry_interval: Duration,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            retries_count: 0,
            min_retry_interval: Duration::from_secs(1),
            max_retry_interval: Duration::from_secs(30 * 60),
        }
    }
}

/// Builder for HTTP client settings
/// Default values are used for any settings that are not explicitly set
/// # Example - Create a new HTTP client with specific settings
/// ```
/// use square_rust::http::client::config::SquareHttpClientConfig;
/// use square_rust::http::headers::Headers;
/// let http_client_configuration =
///    SquareHttpClientConfig::builder()
///       .timeout(15)
///       .user_agent(String::from("some_user_agent"))
///       .default_headers(Headers::default())
///      .build();
/// ```
///
/// # Example - Create a new HTTP client with default settings
/// ```
/// use square_rust::http::client::config::SquareHttpClientConfig;
/// let http_client_configuration = SquareHttpClientConfig::builder().build();
/// ```
#[derive(Default)]
pub struct SquareHttpClientConfigBuilder {
    timeout: Option<u32>,
    user_agent: Option<String>,
    default_headers: Option<Headers>,
    retry_configuration: Option<RetryConfig>,
}

/// Builder for HTTP client settings
impl SquareHttpClientConfigBuilder {
    /// Sets the timeout for HTTP connections
    pub fn timeout(&mut self, timeout: u32) -> &mut Self {
        self.timeout = Some(timeout);
        self
    }

    /// Sets the specific User Agent to use for requests
    pub fn user_agent(&mut self, user_agent: String) -> &mut Self {
        self.user_agent = Some(user_agent);
        self
    }

    /// Sets the default headers to send with each request
    pub fn default_headers(&mut self, default_headers: Headers) -> &mut Self {
        self.default_headers = Some(default_headers);
        self
    }

    /// Sets the retry configuration
    pub fn retry_configuration(&mut self, retry_configuration: RetryConfig) -> &mut Self {
        self.retry_configuration = Some(retry_configuration);
        self
    }

    /// Builds the HTTP client configuration
    pub fn build(&self) -> SquareHttpClientConfig {
        SquareHttpClientConfig {
            timeout: self.timeout.unwrap_or(DEFAULT_TIMEOUT),
            user_agent: self
                .user_agent
                .clone()
                .unwrap_or_else(SquareHttpClientConfig::default_user_agent),
            default_headers: self.default_headers.clone().unwrap_or_default(),
            retry_configuration: self.retry_configuration.clone().unwrap_or_default(),
        }
    }
}

/// Configuration for HTTP client settings
impl SquareHttpClientConfig {
    /// Creates a new builder for HTTP client settings
    pub fn builder() -> SquareHttpClientConfigBuilder {
        SquareHttpClientConfigBuilder::default()
    }

    /// Provides the library/crate's default User Agent
    pub(crate) fn default_user_agent() -> String {
        let sdk_version = env!("CARGO_PKG_VERSION");
        let engine = "Rust";
        let rust_version = rustc_version_runtime::version();
        let os = env::consts::OS;
        format!(
            "Rust Square API Client Lib/({}) {}/{} ({})",
            sdk_version, engine, rust_version, os
        )
    }
}

impl Default for SquareHttpClientConfig {
    /// The default HTTP client settings
    fn default() -> Self {
        Self {
            timeout: DEFAULT_TIMEOUT,
            user_agent: Self::default_user_agent(),
            default_headers: Default::default(),
            retry_configuration: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::http::client::config::{RetryConfig, SquareHttpClientConfig};
    use crate::http::headers::Headers;

    #[test]
    fn http_client_configuration_new_with_default_headers() {
        let http_client_configuration = SquareHttpClientConfig::builder()
            .timeout(15)
            .user_agent(String::from("some_user_agent"))
            .default_headers(Headers::default())
            .build();
        assert_eq!(15, http_client_configuration.timeout);
        assert_eq!(String::from("some_user_agent"), http_client_configuration.user_agent);
        assert_eq!(Headers::default(), http_client_configuration.default_headers);
        assert_eq!(RetryConfig::default(), http_client_configuration.retry_configuration);
    }

    #[test]
    fn http_client_configuration_new_with_different_user_agent_in_headers() {
        let http_client_configuration = SquareHttpClientConfig::builder()
            .timeout(15)
            .user_agent(String::from("some_user_agent"))
            .retry_configuration(RetryConfig::default())
            .build();
        assert_eq!(15, http_client_configuration.timeout);
        assert_eq!(String::from("some_user_agent"), http_client_configuration.user_agent);
        assert_eq!(Headers::default(), http_client_configuration.default_headers);
        assert_eq!(RetryConfig::default(), http_client_configuration.retry_configuration);
    }
}
