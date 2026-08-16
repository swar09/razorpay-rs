use razorpay::{
    Creatable, Fetchable, RazorpayClientBuilder,
    error::RazorpayError,
    models::CreateOrderRequest,
    webhooks::{verify_payment_signature, verify_webhook_signature},
};
use std::time::Duration;
use url::Url;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path},
};

async fn create_test_client(server_uri: &str) -> razorpay::RazorpayClient {
    RazorpayClientBuilder::new()
        .key_id("rzp_test_key")
        .key_secret("test_secret")
        .base_url(Url::parse(server_uri).unwrap())
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap()
}

#[tokio::test]
async fn test_server_returns_wrong_data_type_serde_error() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server.uri()).await;

    // Server returns "amount" as a string "not_a_number" instead of expected u64
    let corrupted_response = serde_json::json!({
        "id": "order_123",
        "entity": "order",
        "amount": "not_a_number",
        "amount_paid": 0,
        "amount_due": 50000,
        "currency": "INR",
        "status": "created",
        "partial_payment": false,
        "attempts": 0,
        "created_at": 1600000000
    });

    Mock::given(method("GET"))
        .and(path("/orders/order_123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(corrupted_response))
        .mount(&mock_server)
        .await;

    let result = client.orders().fetch("order_123", None).await;

    assert!(result.is_err());
    match result {
        Err(RazorpayError::Transport(err)) => {
            assert!(err.is_decode());
        }
        Err(RazorpayError::Serde(_)) => {}
        other => panic!("Expected decode/serde error, got {:?}", other),
    }
}

#[tokio::test]
async fn test_server_returns_partial_error_envelope() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server.uri()).await;

    let minimal_error_body = serde_json::json!({
        "error": {
            "code": "BAD_REQUEST_ERROR",
            "description": "Invalid amount"
        }
    });

    Mock::given(method("POST"))
        .and(path("/orders"))
        .respond_with(ResponseTemplate::new(400).set_body_json(minimal_error_body))
        .mount(&mock_server)
        .await;

    let req = CreateOrderRequest {
        amount: 500,
        currency: "INR".to_string(),
        receipt: None,
        partial_payment: None,
        first_payment_min_amount: None,
        transfers: None,
        notes: None,
    };

    let result = client.orders().create(req, None).await;

    match result {
        Err(RazorpayError::Api(api_err)) => {
            assert_eq!(api_err.code, "BAD_REQUEST_ERROR");
            assert_eq!(api_err.description, "Invalid amount");
            assert_eq!(api_err.field, None);
            assert_eq!(api_err.source, None);
            assert_eq!(api_err.step, None);
            assert_eq!(api_err.reason, None);
        }
        other => panic!("Expected RazorpayError::Api, got {:?}", other),
    }
}

#[tokio::test]
async fn test_webhook_verification_with_malformed_signature_string() {
    let payload = r#"{"event":"payment.authorized"}"#;
    let secret = "secret123";

    let non_hex_signature = "zzzz-invalid-hex-@#$%";

    let result = verify_webhook_signature(payload, non_hex_signature, secret);
    assert!(matches!(result, Err(RazorpayError::SignatureMismatch)));
}

#[tokio::test]
async fn test_webhook_verification_with_empty_strings() {
    let result = verify_webhook_signature("", "any_signature", "secret123");
    assert!(matches!(result, Err(RazorpayError::SignatureMismatch)));

    let result2 = verify_webhook_signature("{}", "", "secret123");
    assert!(matches!(result2, Err(RazorpayError::SignatureMismatch)));
}

#[tokio::test]
async fn test_payment_signature_verification_mismatch() {
    let order_id = "order_123";
    let payment_id = "pay_456";
    let wrong_signature = "1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";
    let secret = "secret";

    let result = verify_payment_signature(order_id, payment_id, wrong_signature, secret);
    assert!(matches!(result, Err(RazorpayError::SignatureMismatch)));
}

#[tokio::test]
async fn test_server_returns_500_html_page_graceful_handling() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server.uri()).await;

    Mock::given(method("GET"))
        .and(path("/orders/order_500"))
        .respond_with(
            ResponseTemplate::new(500)
                .set_body_string("<html><body>500 Internal Server Error</body></html>"),
        )
        .mount(&mock_server)
        .await;

    let result = client.orders().fetch("order_500", None).await;

    match result {
        Err(RazorpayError::Api(api_err)) => {
            assert_eq!(api_err.code, "HTTP_500");
            assert!(api_err.description.contains("500"));
        }
        other => panic!(
            "Expected graceful Api fallback for 500 HTML, got {:?}",
            other
        ),
    }
}
