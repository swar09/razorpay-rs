use razorpay::{
    Fetchable, Listable, RazorpayClientBuilder,
    models::{ContestDisputeRequest, Dispute, DisputePhase, DisputeStatus, RazorpayList},
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
async fn test_disputes_operations() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server.uri()).await;

    let expected_dispute = Dispute {
        id: "disp_123".to_string(),
        entity: "dispute".to_string(),
        payment_id: "pay_123".to_string(),
        amount: 50000,
        currency: "INR".to_string(),
        amount_deducted: 50000,
        reason_code: "fraudulent".to_string(),
        reason_description: "Cardholder claims fraud".to_string(),
        respond_by: 1600003600,
        status: DisputeStatus::Open,
        phase: DisputePhase::Chargeback,
        created_at: 1600000000,
        evidence: None,
    };

    // 1. Fetch Dispute
    Mock::given(method("GET"))
        .and(path("/disputes/disp_123"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_dispute))
        .mount(&mock_server)
        .await;

    let fetched = client
        .disputes()
        .fetch("disp_123", None)
        .await
        .expect("Fetch dispute should succeed");
    assert_eq!(fetched.id, "disp_123");

    // 2. List Disputes
    let list_response = RazorpayList {
        entity: "collection".to_string(),
        count: 1,
        items: vec![expected_dispute.clone()],
    };

    Mock::given(method("GET"))
        .and(path("/disputes"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&list_response))
        .mount(&mock_server)
        .await;

    let list = client
        .disputes()
        .all(None, None)
        .await
        .expect("List disputes should succeed");
    assert_eq!(list.count, 1);
    assert_eq!(list.items[0].id, "disp_123");

    // 3. Accept Dispute
    let mut accepted_dispute = expected_dispute.clone();
    accepted_dispute.status = DisputeStatus::Lost;

    Mock::given(method("POST"))
        .and(path("/disputes/disp_123/accept"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&accepted_dispute))
        .mount(&mock_server)
        .await;

    let accepted = client
        .disputes()
        .accept("disp_123", None)
        .await
        .expect("Accept dispute should succeed");
    assert_eq!(accepted.status, DisputeStatus::Lost);

    // 4. Contest Dispute
    let contest_req = ContestDisputeRequest {
        amount: 50000,
        summary: Some("Order was delivered successfully".to_string()),
        action: "submit".to_string(),
        ..Default::default()
    };

    let mut contested_dispute = expected_dispute;
    contested_dispute.status = DisputeStatus::UnderReview;

    Mock::given(method("PATCH"))
        .and(path("/disputes/disp_123/contest"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .and(body_json(&contest_req))
        .respond_with(ResponseTemplate::new(200).set_body_json(&contested_dispute))
        .mount(&mock_server)
        .await;

    let contested = client
        .disputes()
        .contest("disp_123", contest_req, None)
        .await
        .expect("Contest dispute should succeed");
    assert_eq!(contested.status, DisputeStatus::UnderReview);
}
