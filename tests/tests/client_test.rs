use razorpay::{
    RazorpayClient, RazorpayClientBuilder,
    config::{DEFAULT_BASE_URL, DEFAULT_TIMEOUT},
    error::RazorpayError,
};
use std::time::Duration;
use url::Url;

#[test]
fn test_client_new_default_config() {
    let key = "rzp_test_key123";
    let secret = "test_secret_456";

    let client = RazorpayClient::new(key, secret).expect("Client should build successfully");

    assert_eq!(client.config().key_id, key);
    assert_eq!(client.config().key_secret, secret);
    assert_eq!(client.config().base_url.as_str(), DEFAULT_BASE_URL);
    assert_eq!(client.config().timeout, DEFAULT_TIMEOUT);
}

#[test]
fn test_client_builder_custom_config() {
    let key = "custom_key";
    let secret = "custom_secret";
    let custom_url = Url::parse("https://custom.razorpay.local/v1").unwrap();
    let custom_timeout = Duration::from_secs(10);

    let client = RazorpayClientBuilder::new()
        .key_id(key)
        .key_secret(secret)
        .base_url(custom_url.clone())
        .timeout(custom_timeout)
        .build()
        .expect("Client should build successfully with custom config");

    assert_eq!(client.config().key_id, key);
    assert_eq!(client.config().key_secret, secret);
    assert_eq!(client.config().base_url, custom_url);
    assert_eq!(client.config().timeout, custom_timeout);
}

#[test]
fn test_client_builder_missing_key_id() {
    let result = RazorpayClientBuilder::new()
        .key_secret("some_secret")
        .build();

    assert!(matches!(
        result,
        Err(RazorpayError::Config("missing key_id"))
    ));
}

#[test]
fn test_client_builder_missing_key_secret() {
    let result = RazorpayClientBuilder::new().key_id("some_key").build();

    assert!(matches!(
        result,
        Err(RazorpayError::Config("missing key_secret"))
    ));
}

#[test]
fn test_client_is_clone_and_send_sync() {
    fn assert_send_sync<T: Send + Sync + Clone>() {}
    assert_send_sync::<RazorpayClient>();
}

#[tokio::test]
async fn test_client_with_account_sub_merchant() {
    let client = RazorpayClient::new("rzp_test_key", "test_secret").unwrap();
    let sub_client = client.with_account("acc_sub12345").unwrap();

    assert_eq!(sub_client.config().key_id, "rzp_test_key");
}
