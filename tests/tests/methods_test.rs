use razorpay::{RazorpayClientBuilder, models::PaymentMethods};
use std::time::Duration;
use url::Url;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{basic_auth, method, path},
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
async fn test_methods_operations() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server.uri()).await;

    let expected_methods = serde_json::json!({
        "entity": "methods",
        "card": true,
        "debit_card": true,
        "credit_card": true,
        "netbanking": {
            "HDFC": "HDFC Bank",
            "ICIC": "ICICI Bank",
            "SBIN": "State Bank of India"
        },
        "wallet": {
            "payzapp": "PayZapp",
            "olamoney": "Ola Money"
        },
        "upi": true,
        "card_networks": {
            "Visa": "1",
            "MasterCard": "1"
        }
    });

    Mock::given(method("GET"))
        .and(path("/methods"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_methods))
        .mount(&mock_server)
        .await;

    // Test via client.methods().all()
    let methods: PaymentMethods = client
        .methods()
        .all(None)
        .await
        .expect("Fetch methods should succeed");

    assert_eq!(methods.entity, "methods");
    assert!(methods.card);
    assert_eq!(methods.debit_card, Some(true));
    assert!(methods.netbanking.is_some());

    // Test via client.payments().methods()
    let payment_methods: PaymentMethods = client
        .payments()
        .methods(None)
        .await
        .expect("Payments methods helper should succeed");

    assert_eq!(payment_methods.entity, "methods");
    assert!(payment_methods.card);
}
