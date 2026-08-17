use razorpay::{
    Creatable, Fetchable, Listable, RazorpayClientBuilder, Updatable,
    models::{CreateRefundRequest, RazorpayList, Refund, RefundSpeed, UpdateRefundRequest},
};
use std::{collections::HashMap, time::Duration};
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
async fn test_refunds_crud_operations() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server.uri()).await;

    let create_req = CreateRefundRequest {
        payment_id: Some("pay_12345".to_string()),
        amount: Some(5000),
        speed: Some(RefundSpeed::Instant),
        notes: None,
        receipt: None,
    };

    let expected_refund = Refund {
        id: "rfnd_standalone_99".to_string(),
        entity: "refund".to_string(),
        amount: 5000,
        currency: "INR".to_string(),
        payment_id: "pay_12345".to_string(),
        notes: None,
        receipt: None,
        acquirer_data: None,
        created_at: 1600000000,
        batch_id: None,
        status: razorpay::models::RefundStatus::Processed,
        speed_processed: Some(RefundSpeed::Instant),
        speed_requested: Some(RefundSpeed::Instant),
    };

    // 1. Create Standalone Refund
    Mock::given(method("POST"))
        .and(path("/refunds"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .and(body_json(&create_req))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_refund))
        .mount(&mock_server)
        .await;

    let created = client
        .refunds()
        .create(create_req, None)
        .await
        .expect("Refund creation should succeed");
    assert_eq!(created.id, "rfnd_standalone_99");

    // 2. Fetch Refund by ID
    Mock::given(method("GET"))
        .and(path("/refunds/rfnd_standalone_99"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_refund))
        .mount(&mock_server)
        .await;

    let fetched = client
        .refunds()
        .fetch("rfnd_standalone_99", None)
        .await
        .expect("Refund fetch should succeed");
    assert_eq!(fetched.id, "rfnd_standalone_99");

    // 3. List All Refunds
    let list_response = RazorpayList {
        entity: "collection".to_string(),
        count: 1,
        items: vec![expected_refund.clone()],
    };

    Mock::given(method("GET"))
        .and(path("/refunds"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&list_response))
        .mount(&mock_server)
        .await;

    let list = client
        .refunds()
        .all(None, None)
        .await
        .expect("Refunds list should succeed");
    assert_eq!(list.count, 1);
    assert_eq!(list.items[0].id, "rfnd_standalone_99");

    // 4. Update Refund Notes
    let mut update_notes = HashMap::new();
    update_notes.insert("reason".to_string(), "customer requested".to_string());
    let update_req = UpdateRefundRequest {
        notes: update_notes.clone().into(),
    };

    let mut updated_refund = expected_refund;
    updated_refund.notes = Some(update_notes.into());

    Mock::given(method("PATCH"))
        .and(path("/refunds/rfnd_standalone_99"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .and(body_json(&update_req))
        .respond_with(ResponseTemplate::new(200).set_body_json(&updated_refund))
        .mount(&mock_server)
        .await;

    let updated = client
        .refunds()
        .update("rfnd_standalone_99", update_req, None)
        .await
        .expect("Refund update should succeed");
    assert!(updated.notes.is_some());
}
