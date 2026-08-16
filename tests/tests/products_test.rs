use razorpay::{
    models::{ProductConfiguration, TncResponse},
    RazorpayClientBuilder,
};
use std::time::Duration;
use url::Url;
use wiremock::{
    matchers::{basic_auth, method, path},
    Mock, MockServer, ResponseTemplate,
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
async fn test_products_and_tnc_operations() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server.uri()).await;

    let expected_prod = ProductConfiguration {
        id: Some("prod_123".to_string()),
        account_id: Some("acc_123".to_string()),
        product_name: Some("payment_gateway".to_string()),
        status: Some("active".to_string()),
        configuration: Some(serde_json::json!({"settlements": {"schedule": "t+2"}})),
        requirements: None,
    };

    // 1. Request Configuration
    Mock::given(method("POST"))
        .and(path("/v2/accounts/acc_123/products"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_prod))
        .mount(&mock_server)
        .await;

    let created = client
        .products()
        .request_configuration("acc_123", &serde_json::json!({"product_name": "payment_gateway"}), None)
        .await
        .expect("Request configuration should succeed");
    assert_eq!(created.id, Some("prod_123".to_string()));

    // 2. Fetch Product Configuration
    Mock::given(method("GET"))
        .and(path("/v2/accounts/acc_123/products/prod_123"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_prod))
        .mount(&mock_server)
        .await;

    let fetched = client
        .products()
        .fetch("acc_123", "prod_123", None)
        .await
        .expect("Fetch configuration should succeed");
    assert_eq!(fetched.product_name, Some("payment_gateway".to_string()));

    // 3. Update Product Configuration
    Mock::given(method("PATCH"))
        .and(path("/v2/accounts/acc_123/products/prod_123"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_prod))
        .mount(&mock_server)
        .await;

    let updated = client
        .products()
        .update("acc_123", "prod_123", &serde_json::json!({"settlements": {"schedule": "t+1"}}), None)
        .await
        .expect("Update configuration should succeed");
    assert_eq!(updated.status, Some("active".to_string()));

    // 4. Fetch TNC
    let expected_tnc = TncResponse {
        entity: Some("tnc".to_string()),
        product_name: Some("payment_gateway".to_string()),
        tnc: Some(serde_json::json!({"content": "Standard Terms and Conditions"})),
        last_updated_at: Some(1700000000),
    };

    Mock::given(method("GET"))
        .and(path("/v2/products/payment_gateway/tnc"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_tnc))
        .mount(&mock_server)
        .await;

    let tnc = client
        .products()
        .fetch_tnc("payment_gateway", None)
        .await
        .expect("Fetch TNC should succeed");
    assert_eq!(tnc.product_name, Some("payment_gateway".to_string()));
}
