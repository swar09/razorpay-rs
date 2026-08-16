use thiserror::Error;

/// Main error enum for all operations in the Razorpay SDK.
#[derive(Debug, Error)]
pub enum RazorpayError {
    /// Underlying HTTP transport or connection error.
    #[error("transport error: {0}")]
    Transport(#[from] reqwest::Error),

    /// Error returned by the Razorpay API (4xx / 5xx responses).
    #[error("razorpay api error: {0:?}")]
    Api(Box<crate::models::RazorpayError>),

    /// JSON serialization or deserialization failure.
    #[error("failed to (de)serialize: {0}")]
    Serde(#[from] serde_json::Error),

    /// Invalid argument or parameter passed to an SDK method.
    #[error("invalid input: {0}")]
    InvalidInput(String),

    /// Webhook or payment HMAC-SHA256 signature mismatch.
    #[error("webhook/payment signature did not match")]
    SignatureMismatch,

    /// Missing required client configuration (e.g. missing key_id or key_secret).
    #[error("missing required client config: {0}")]
    Config(&'static str),

    /// URL parsing error.
    #[error("invalid URL: {0}")]
    Url(#[from] url::ParseError),

    /// Standard I/O error (e.g. reading files for document uploads).
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

impl From<crate::models::RazorpayError> for RazorpayError {
    fn from(err: crate::models::RazorpayError) -> Self {
        RazorpayError::Api(Box::new(err))
    }
}

/// Specialized Result alias for Razorpay SDK operations.
pub type RazorpayResult<T> = Result<T, RazorpayError>;
