use razorpay::{
    models::{
        CapturePaymentRequest, CardDetails, CreateRefundRequest, DowntimeInstrument, Payment,
        PaymentDowntime, PaymentMethod, PaymentStatus, Refund, RefundSpeed, UpdatePaymentRequest,
    },
    Fetchable, RazorpayClientBuilder, Updatable,
};
use std::{collections::HashMap, time::Duration};
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
async fn test_payments_fetch_success() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server.uri()).await;

    let expected_payment = Payment {
        id: "pay_29QQoUBi66xm2f".to_string(),
        entity: "payment".to_string(),
        amount: 50000,
        currency: "INR".to_string(),
        status: PaymentStatus::Authorized,
        order_id: Some("order_Hk1234567890".to_string()),
        invoice_id: None,
        international: false,
        method: Some(PaymentMethod::Card),
        amount_refunded: 0,
        refund_status: None,
        captured: false,
        description: Some("Test Payment".to_string()),
        card_id: None,
        card: None,
        bank: None,
        wallet: None,
        vpa: None,
        email: Some("customer@example.com".to_string()),
        contact: Some("+919999999999".to_string()),
        customer_id: None,
        token_id: None,
        notes: None,
        fee: None,
        tax: None,
        error_code: None,
        error_description: None,
        error_source: None,
        error_step: None,
        error_reason: None,
        acquirer_data: None,
        created_at: 1600000000,
        emi: None,
        reward: None,
        upi: None,
        base_amount: None,
    };

    Mock::given(method("GET"))
        .and(path("/payments/pay_29QQoUBi66xm2f"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_payment))
        .mount(&mock_server)
        .await;

    let payment = client
        .payments()
        .fetch("pay_29QQoUBi66xm2f", None)
        .await
        .expect("Payment fetch should succeed");

    assert_eq!(payment.id, "pay_29QQoUBi66xm2f");
    assert_eq!(payment.status, PaymentStatus::Authorized);
}

#[tokio::test]
async fn test_payments_capture_success() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server.uri()).await;

    let capture_request = CapturePaymentRequest {
        amount: 50000,
        currency: "INR".to_string(),
    };

    let expected_payment = Payment {
        id: "pay_29QQoUBi66xm2f".to_string(),
        entity: "payment".to_string(),
        amount: 50000,
        currency: "INR".to_string(),
        status: PaymentStatus::Captured,
        order_id: Some("order_Hk1234567890".to_string()),
        invoice_id: None,
        international: false,
        method: Some(PaymentMethod::Card),
        amount_refunded: 0,
        refund_status: None,
        captured: true,
        description: Some("Test Payment".to_string()),
        card_id: None,
        card: None,
        bank: None,
        wallet: None,
        vpa: None,
        email: Some("customer@example.com".to_string()),
        contact: Some("+919999999999".to_string()),
        customer_id: None,
        token_id: None,
        notes: None,
        fee: None,
        tax: None,
        error_code: None,
        error_description: None,
        error_source: None,
        error_step: None,
        error_reason: None,
        acquirer_data: None,
        created_at: 1600000000,
        emi: None,
        reward: None,
        upi: None,
        base_amount: None,
    };

    Mock::given(method("POST"))
        .and(path("/payments/pay_29QQoUBi66xm2f/capture"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .and(body_json(&capture_request))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_payment))
        .mount(&mock_server)
        .await;

    let captured = client
        .payments()
        .capture("pay_29QQoUBi66xm2f", capture_request, None)
        .await
        .expect("Payment capture should succeed");

    assert_eq!(captured.id, "pay_29QQoUBi66xm2f");
    assert_eq!(captured.status, PaymentStatus::Captured);
    assert!(captured.captured);
}

#[tokio::test]
async fn test_payments_update_notes() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server.uri()).await;

    let mut notes = HashMap::new();
    notes.insert("internal_id".to_string(), "abc_999".to_string());
    let update_req = UpdatePaymentRequest {
        notes: notes.clone().into(),
    };

    let expected_payment = Payment {
        id: "pay_12345".to_string(),
        entity: "payment".to_string(),
        amount: 25000,
        currency: "INR".to_string(),
        status: PaymentStatus::Captured,
        order_id: None,
        invoice_id: None,
        international: false,
        method: Some(PaymentMethod::Card),
        amount_refunded: 0,
        refund_status: None,
        captured: true,
        description: None,
        card_id: None,
        card: None,
        bank: None,
        wallet: None,
        vpa: None,
        email: Some("test@example.com".to_string()),
        contact: Some("+919876543210".to_string()),
        customer_id: None,
        token_id: None,
        notes: Some(notes.into()),
        fee: None,
        tax: None,
        error_code: None,
        error_description: None,
        error_source: None,
        error_step: None,
        error_reason: None,
        acquirer_data: None,
        created_at: 1600000000,
        emi: None,
        reward: None,
        upi: None,
        base_amount: None,
    };

    Mock::given(method("PATCH"))
        .and(path("/payments/pay_12345"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .and(body_json(&update_req))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_payment))
        .mount(&mock_server)
        .await;

    let updated = client
        .payments()
        .update("pay_12345", update_req, None)
        .await
        .expect("Payment update should succeed");

    assert_eq!(updated.id, "pay_12345");
    assert!(updated.notes.is_some());
}

#[tokio::test]
async fn test_payments_refund_endpoint() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server.uri()).await;

    let refund_req = CreateRefundRequest {
        payment_id: None,
        amount: Some(10000),
        speed: Some(RefundSpeed::Normal),
        notes: None,
        receipt: Some("ref_rcpt_1".to_string()),
    };

    let expected_refund = Refund {
        id: "rfnd_12345".to_string(),
        entity: "refund".to_string(),
        amount: 10000,
        currency: "INR".to_string(),
        payment_id: "pay_12345".to_string(),
        notes: None,
        receipt: Some("ref_rcpt_1".to_string()),
        acquirer_data: None,
        created_at: 1600000000,
        batch_id: None,
        status: "processed".to_string(),
        speed_processed: RefundSpeed::Normal,
        speed_requested: RefundSpeed::Normal,
    };

    Mock::given(method("POST"))
        .and(path("/payments/pay_12345/refund"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .and(body_json(&refund_req))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_refund))
        .mount(&mock_server)
        .await;

    let refund = client
        .payments()
        .refund("pay_12345", refund_req, None)
        .await
        .expect("Payment refund should succeed");

    assert_eq!(refund.id, "rfnd_12345");
    assert_eq!(refund.amount, 10000);
}

#[tokio::test]
async fn test_payments_card_details() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server.uri()).await;

    let expected_card = CardDetails {
        id: "card_123".to_string(),
        entity: "card".to_string(),
        name: "Gaurav Kumar".to_string(),
        last4: "1111".to_string(),
        network: "Visa".to_string(),
        card_type: "credit".to_string(),
        issuer: Some("HDFC".to_string()),
        international: false,
        emi: true,
        sub_type: Some("consumer".to_string()),
        token_iin: None,
        fingerprint: None,
    };

    Mock::given(method("GET"))
        .and(path("/payments/pay_12345/card"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_card))
        .mount(&mock_server)
        .await;

    let card = client
        .payments()
        .card_details("pay_12345", None)
        .await
        .expect("Card details fetch should succeed");

    assert_eq!(card.last4, "1111");
    assert_eq!(card.network, "Visa");
}

#[tokio::test]
async fn test_payments_downtime() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server.uri()).await;

    let expected_downtime = PaymentDowntime {
        id: "down_123".to_string(),
        entity: "payment.downtime".to_string(),
        method: "card".to_string(),
        begin: 1600000000,
        end: Some(1600003600),
        status: "scheduled".to_string(),
        scheduled: true,
        severity: "high".to_string(),
        instrument: Some(DowntimeInstrument {
            bank: Some("HDFC".to_string()),
            psp: None,
            issuer: None,
            network: Some("Visa".to_string()),
        }),
        created_at: 1600000000,
        updated_at: 1600000000,
    };

    Mock::given(method("GET"))
        .and(path("/payments/downtimes/down_123"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_downtime))
        .mount(&mock_server)
        .await;

    let downtime = client
        .payments()
        .fetch_downtime_by_id("down_123", None)
        .await
        .expect("Downtime fetch should succeed");

    assert_eq!(downtime.id, "down_123");
    assert_eq!(downtime.severity, "high");
}

#[tokio::test]
async fn test_payments_otp_and_transfers() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server.uri()).await;

    // 1. OTP Generate
    Mock::given(method("POST"))
        .and(path("/payments/pay_123/otp_generate"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({"status": "otp_sent"})))
        .mount(&mock_server)
        .await;

    let gen_res = client
        .payments()
        .otp_generate("pay_123", None)
        .await
        .expect("OTP generate should succeed");
    assert_eq!(gen_res["status"], "otp_sent");

    // 2. OTP Submit
    Mock::given(method("POST"))
        .and(path("/payments/pay_123/otp_submit"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({"status": "authenticated"})))
        .mount(&mock_server)
        .await;

    let sub_res = client
        .payments()
        .otp_submit("pay_123", &serde_json::json!({"otp": "123456"}), None)
        .await
        .expect("OTP submit should succeed");
    assert_eq!(sub_res["status"], "authenticated");
}
