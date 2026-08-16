use razorpay::{
    Creatable, Fetchable, Listable, RazorpayClientBuilder,
    models::{CreateQrCodeRequest, QrCode, QrCodeStatus, RazorpayList},
};
use std::time::Duration;
use url::Url;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{basic_auth, body_json, method, path},
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
async fn test_qr_codes_operations() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server.uri()).await;

    let create_req = CreateQrCodeRequest {
        qr_type: "upi_qr".to_string(),
        name: Some("Store Front QR".to_string()),
        usage: "single_use".to_string(),
        fixed_amount: true,
        payment_amount: Some(50000),
        description: Some("Order #123 QR".to_string()),
        customer_id: None,
        close_by: None,
        notes: None,
    };

    let expected_qr = QrCode {
        id: "qr_123".to_string(),
        entity: "qr_code".to_string(),
        created_at: 1600000000,
        close_by: None,
        close_reason: None,
        closed_at: None,
        customer_id: None,
        description: Some("Order #123 QR".to_string()),
        fixed_amount: true,
        image_url: "https://rzp.io/i/qr123.png".to_string(),
        name: Some("Store Front QR".to_string()),
        notes: None,
        payment_amount: Some(50000),
        payments_amount_received: 0,
        payments_count_received: 0,
        status: QrCodeStatus::Active,
        qr_type: "upi_qr".to_string(),
        usage: "single_use".to_string(),
    };

    // 1. Create QR Code
    Mock::given(method("POST"))
        .and(path("/payments/qr_codes"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .and(body_json(&create_req))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_qr))
        .mount(&mock_server)
        .await;

    let qr = client
        .qr_codes()
        .create(create_req, None)
        .await
        .expect("Create QR code should succeed");
    assert_eq!(qr.id, "qr_123");
    assert_eq!(qr.status, QrCodeStatus::Active);

    // 2. Fetch QR Code
    Mock::given(method("GET"))
        .and(path("/payments/qr_codes/qr_123"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_qr))
        .mount(&mock_server)
        .await;

    let fetched = client
        .qr_codes()
        .fetch("qr_123", None)
        .await
        .expect("Fetch QR code should succeed");
    assert_eq!(fetched.id, "qr_123");

    // 3. List QR Codes
    let list_response = RazorpayList {
        entity: "collection".to_string(),
        count: 1,
        items: vec![expected_qr.clone()],
    };

    Mock::given(method("GET"))
        .and(path("/payments/qr_codes"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&list_response))
        .mount(&mock_server)
        .await;

    let list = client
        .qr_codes()
        .all(None, None)
        .await
        .expect("List QR codes should succeed");
    assert_eq!(list.count, 1);
    assert_eq!(list.items[0].id, "qr_123");

    // 4. Close QR Code
    let mut closed_qr = expected_qr;
    closed_qr.status = QrCodeStatus::Closed;

    Mock::given(method("POST"))
        .and(path("/payments/qr_codes/qr_123/close"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&closed_qr))
        .mount(&mock_server)
        .await;

    let closed = client
        .qr_codes()
        .close("qr_123", None)
        .await
        .expect("Close QR code should succeed");
    assert_eq!(closed.status, QrCodeStatus::Closed);
}
