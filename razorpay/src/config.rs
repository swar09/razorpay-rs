use std::{fmt, time::Duration};
use url::Url;

/// Default Razorpay API base URL.
pub const DEFAULT_BASE_URL: &str = "https://api.razorpay.com/v1";

/// Default HTTP request timeout (30 seconds).
pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

/// Runtime configuration for the Razorpay client.
#[derive(Clone)]
pub struct RazorpayConfig {
    /// Razorpay Key ID (API Key).
    pub key_id: String,
    /// Razorpay Key Secret.
    pub key_secret: String,
    /// Base URL for API requests.
    pub base_url: Url,
    /// Request timeout duration.
    pub timeout: Duration,
}

impl std::fmt::Debug for RazorpayConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RazorpayConfig")
            .field("key_id", &self.key_id)
            .field("key_secret", &"[REDACTED]")
            .field("base_url", &self.base_url)
            .field("timeout", &self.timeout)
            .finish()
    }
}

impl RazorpayConfig {
    /// Create a new configuration with default base URL and timeout.
    pub fn new(
        key_id: impl Into<String>,
        key_secret: impl Into<String>,
    ) -> Result<Self, url::ParseError> {
        let base_url = Url::parse(DEFAULT_BASE_URL)?;
        Ok(Self {
            key_id: key_id.into(),
            key_secret: key_secret.into(),
            base_url,
            timeout: DEFAULT_TIMEOUT,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug_redacts_key_secret() {
        let config = RazorpayConfig {
            key_id: "rzp_test_key".to_string(),
            key_secret: "super_secret_value".to_string(),
            base_url: Url::parse(DEFAULT_BASE_URL).unwrap(),
            timeout: DEFAULT_TIMEOUT,
        };

        let debug = format!("{config:?}");
        assert!(!debug.contains("super_secret_value"));
        assert!(debug.contains("[REDACTED]"));
    }
}
