use razorpay::{
    models::{CreatePaymentLinkRequest, NotifyMedium, PaymentLink},
    Creatable, RazorpayClientBuilder,
};
use std::time::Duration;
use url::Url;
use wiremock::{
    matchers::{basic_auth, body_json, method, path},
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
async fn test_payment_links_operations() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server.uri()).await;

    let create_link_req = CreatePaymentLinkRequest {
        amount: 1000,
        currency: Some("INR".to_string()),
        accept_partial: Some(false),
        first_min_partial_amount: None,
        expire_by: None,
        reference_id: Some("ref_plink_01".to_string()),
        description: Some("Test Link".to_string()),
        customer: None,
        notify: None,
        reminder_enable: None,
        notes: None,
        callback_url: None,
        callback_method: None,
    };

    let expected_link = PaymentLink {
        id: "plink_123".to_string(),
        entity: Some("payment_link".to_string()),
        accept_partial: false,
        amount: 1000,
        amount_paid: 0,
        cancelled_at: None,
        created_at: 1600000000,
        currency: "INR".to_string(),
        customer_id: None,
        description: Some("Test Link".to_string()),
        expire_by: None,
        expired_at: None,
        first_min_partial_amount: None,
        notes: None,
        notify: None,
        payments: None,
        reference_id: Some("ref_plink_01".to_string()),
        reminder_enable: false,
        short_url: "https://rzp.io/i/test123".to_string(),
        status: "created".to_string(),
        updated_at: 1600000000,
        upi_link: false,
        user_id: None,
        callback_url: None,
        callback_method: None,
    };

    // 1. Create Payment Link
    Mock::given(method("POST"))
        .and(path("/payment_links"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .and(body_json(&create_link_req))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_link))
        .mount(&mock_server)
        .await;

    let plink = client
        .payment_links()
        .create(create_link_req, None)
        .await
        .expect("Create payment link should succeed");
    assert_eq!(plink.id, "plink_123");
    assert_eq!(plink.short_url, "https://rzp.io/i/test123");

    // 2. Cancel Payment Link
    let mut cancelled_link = expected_link.clone();
    cancelled_link.status = "cancelled".to_string();

    Mock::given(method("POST"))
        .and(path("/payment_links/plink_123/cancel"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&cancelled_link))
        .mount(&mock_server)
        .await;

    let cancelled = client
        .payment_links()
        .cancel("plink_123", None)
        .await
        .expect("Cancel payment link should succeed");
    assert_eq!(cancelled.status, "cancelled");

    // 3. Notify by SMS
    Mock::given(method("POST"))
        .and(path("/payment_links/plink_123/notify_by/sms"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({"success": true})))
        .mount(&mock_server)
        .await;

    let notify_resp = client
        .payment_links()
        .notify_by("plink_123", NotifyMedium::Sms, None)
        .await
        .expect("Notify by SMS should succeed");
    assert_eq!(notify_resp["success"], true);
}
