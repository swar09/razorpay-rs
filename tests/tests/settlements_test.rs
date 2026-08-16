use razorpay::{
    Fetchable, Listable, RazorpayClientBuilder,
    models::{
        CreateInstantSettlementRequest, InstantSettlement, RazorpayList, Settlement,
        SettlementReconItem,
    },
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
async fn test_settlements_operations() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server.uri()).await;

    let expected_settlement = Settlement {
        id: "setl_123".to_string(),
        entity: "settlement".to_string(),
        amount: 100000,
        status: razorpay::models::SettlementStatus::Processed,
        fees: 2000,
        tax: 360,
        utr: Some("UTR123456789".to_string()),
        created_at: 1600000000,
    };

    // 1. Fetch Settlement
    Mock::given(method("GET"))
        .and(path("/settlements/setl_123"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_settlement))
        .mount(&mock_server)
        .await;

    let fetched = client
        .settlements()
        .fetch("setl_123", None)
        .await
        .expect("Fetch settlement should succeed");
    assert_eq!(fetched.id, "setl_123");
    assert_eq!(fetched.amount, 100000);

    // 2. List Settlements
    let list_response = RazorpayList {
        entity: "collection".to_string(),
        count: 1,
        items: vec![expected_settlement],
    };

    Mock::given(method("GET"))
        .and(path("/settlements"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&list_response))
        .mount(&mock_server)
        .await;

    let list = client
        .settlements()
        .all(None, None)
        .await
        .expect("List settlements should succeed");
    assert_eq!(list.count, 1);
    assert_eq!(list.items[0].id, "setl_123");

    // 3. Settlement Reports
    let recon_item = SettlementReconItem {
        entity_id: "pay_123".to_string(),
        transaction_type: "payment".to_string(),
        amount: 50000,
        fee: 1000,
        tax: 180,
        debit: 0,
        credit: 48820,
        currency: "INR".to_string(),
        settled: true,
        created_at: 1600000000,
        settled_at: Some(1600003600),
        settlement_id: Some("setl_123".to_string()),
        description: None,
        notes: None,
        ..Default::default()
    };

    let reports_response = RazorpayList {
        entity: "collection".to_string(),
        count: 1,
        items: vec![recon_item],
    };

    Mock::given(method("GET"))
        .and(path("/settlements/recon/combined"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&reports_response))
        .mount(&mock_server)
        .await;

    let reports = client
        .settlements()
        .reports(None, None)
        .await
        .expect("Settlement reports should succeed");
    assert_eq!(reports.items[0].entity_id, "pay_123");

    // 4. Create On-demand Settlement
    let ondemand_req = CreateInstantSettlementRequest {
        amount: 50000,
        settle_full_balance: Some(false),
        description: None,
        notes: None,
    };

    let expected_ondemand = InstantSettlement {
        id: "setl_ondemand_1".to_string(),
        entity: "settlement.ondemand".to_string(),
        amount: 50000,
        amount_settled: 50000,
        fees: 500,
        tax: 90,
        currency: "INR".to_string(),
        settle_full_balance: false,
        status: "created".to_string(),
        description: None,
        notes: None,
        scheduled: None,
        created_at: 1600000000,
        ondemand_payouts: None,
    };

    Mock::given(method("POST"))
        .and(path("/settlements/ondemand"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .and(body_json(&ondemand_req))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_ondemand))
        .mount(&mock_server)
        .await;

    let ondemand = client
        .settlements()
        .create_ondemand(ondemand_req, None)
        .await
        .expect("Create ondemand settlement should succeed");
    assert_eq!(ondemand.id, "setl_ondemand_1");
}
