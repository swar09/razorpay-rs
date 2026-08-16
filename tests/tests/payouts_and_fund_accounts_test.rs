use razorpay::{
    Creatable, Fetchable, Listable, RazorpayClientBuilder,
    models::{CreateFundAccountRequest, CreatePayoutRequest, FundAccount, Payout, RazorpayList},
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
async fn test_fund_accounts_and_payouts_operations() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server.uri()).await;

    // 1. Create Fund Account
    let fund_req = CreateFundAccountRequest {
        contact_id: "cont_123".to_string(),
        account_type: "bank_account".to_string(),
        bank_account: Some(serde_json::json!({
            "name": "Gaurav Kumar",
            "account_number": "11214311215411",
            "ifsc": "HDFC0000053"
        })),
        ..Default::default()
    };

    let expected_fund_acc = FundAccount {
        id: "fa_123".to_string(),
        entity: "fund_account".to_string(),
        contact_id: "cont_123".to_string(),
        account_type: "bank_account".to_string(),
        active: true,
        bank_account: fund_req.bank_account.clone(),
        vpa: None,
        card: None,
        wallet: None,
        created_at: 1600000000,
    };

    Mock::given(method("POST"))
        .and(path("/fund_accounts"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .and(body_json(&fund_req))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_fund_acc))
        .mount(&mock_server)
        .await;

    let fund_acc = client
        .fund_accounts()
        .create(fund_req, None)
        .await
        .expect("Create fund account should succeed");
    assert_eq!(fund_acc.id, "fa_123");

    // 2. Fetch Fund Account
    Mock::given(method("GET"))
        .and(path("/fund_accounts/fa_123"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_fund_acc))
        .mount(&mock_server)
        .await;

    let fetched_fa = client
        .fund_accounts()
        .fetch("fa_123", None)
        .await
        .expect("Fetch fund account should succeed");
    assert_eq!(fetched_fa.id, "fa_123");

    // 3. Create Payout
    let payout_req = CreatePayoutRequest {
        account_number: "7878780080316316".to_string(),
        fund_account_id: "fa_123".to_string(),
        amount: 100000,
        currency: "INR".to_string(),
        mode: "IMPS".to_string(),
        purpose: "payout".to_string(),
        queue_if_low_balance: Some(true),
        reference_id: Some("payout_ref_001".to_string()),
        narration: Some("Vendor Payout".to_string()),
        notes: None,
    };

    let expected_payout = Payout {
        id: "pout_123".to_string(),
        entity: "payout".to_string(),
        fund_account_id: "fa_123".to_string(),
        amount: 100000,
        currency: "INR".to_string(),
        notes: None,
        fees: Some(500),
        tax: Some(90),
        status: "processing".to_string(),
        purpose: Some("payout".to_string()),
        utr: None,
        mode: "IMPS".to_string(),
        reference_id: Some("payout_ref_001".to_string()),
        narration: Some("Vendor Payout".to_string()),
        created_at: 1600000000,
    };

    Mock::given(method("POST"))
        .and(path("/payouts"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .and(body_json(&payout_req))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_payout))
        .mount(&mock_server)
        .await;

    let payout = client
        .payouts()
        .create(payout_req, None)
        .await
        .expect("Create payout should succeed");
    assert_eq!(payout.id, "pout_123");

    // 4. Cancel Payout
    let mut cancelled_payout = expected_payout.clone();
    cancelled_payout.status = "cancelled".to_string();

    Mock::given(method("POST"))
        .and(path("/payouts/pout_123/cancel"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&cancelled_payout))
        .mount(&mock_server)
        .await;

    let cancelled = client
        .payouts()
        .cancel("pout_123", None)
        .await
        .expect("Cancel payout should succeed");
    assert_eq!(cancelled.status, "cancelled");

    // 5. List Payouts
    let payouts_list = RazorpayList {
        entity: "collection".to_string(),
        count: 1,
        items: vec![expected_payout],
    };

    Mock::given(method("GET"))
        .and(path("/payouts"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&payouts_list))
        .mount(&mock_server)
        .await;

    let list = client
        .payouts()
        .all(None, None)
        .await
        .expect("List payouts should succeed");
    assert_eq!(list.count, 1);
    assert_eq!(list.items[0].id, "pout_123");
}
