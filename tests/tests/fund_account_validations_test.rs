use razorpay::{
    Creatable, Listable, RazorpayClientBuilder,
    models::{
        CreateFundAccountValidationRequest, FundAccountValidation, FundAccountValidationTarget,
        ListOptions, RazorpayList,
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
async fn test_fund_account_validations_operations() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server.uri()).await;

    let expected_validation = serde_json::json!({
        "id": "fav_00000000000001",
        "entity": "fund_account.validation",
        "fund_account_id": "fa_00000000000001",
        "status": "completed",
        "amount": 100,
        "currency": "INR",
        "results": {
            "registered_name": "Gaurav Kumar",
            "account_status": "active",
            "name_match_score": 100.0
        },
        "created_at": 1600000000
    });

    let create_payload = CreateFundAccountValidationRequest {
        fund_account: FundAccountValidationTarget {
            id: Some("fa_00000000000001".to_string()),
        },
        amount: 100,
        currency: "INR".to_string(),
        notes: None,
    };

    // 1. Create Fund Account Validation
    Mock::given(method("POST"))
        .and(path("/fund_accounts/validations"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .and(body_json(&create_payload))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_validation))
        .mount(&mock_server)
        .await;

    let val: FundAccountValidation = client
        .fund_accounts()
        .validations()
        .create(create_payload.clone(), None)
        .await
        .expect("Create fund account validation should succeed");

    assert_eq!(val.id, "fav_00000000000001");
    assert_eq!(val.status, "completed");
    assert_eq!(val.amount, 100);

    // 2. Fetch Fund Account Validation by ID
    Mock::given(method("GET"))
        .and(path("/fund_accounts/validations/fav_00000000000001"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_validation))
        .mount(&mock_server)
        .await;

    let val_fetch: FundAccountValidation = client
        .fund_accounts()
        .fetch_validation("fav_00000000000001", None)
        .await
        .expect("Fetch fund account validation should succeed");

    assert_eq!(val_fetch.id, "fav_00000000000001");
    assert_eq!(
        val_fetch.results.and_then(|r| r.registered_name),
        Some("Gaurav Kumar".to_string())
    );

    // 3. List all Fund Account Validations
    let list_response = RazorpayList {
        entity: "collection".to_string(),
        count: 1,
        items: vec![serde_json::from_value::<FundAccountValidation>(expected_validation).unwrap()],
    };

    Mock::given(method("GET"))
        .and(path("/fund_accounts/validations"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&list_response))
        .mount(&mock_server)
        .await;

    let list = client
        .fund_accounts()
        .validations()
        .all(
            Some(ListOptions {
                count: Some(10),
                skip: None,
                from: None,
                to: None,
            }),
            None,
        )
        .await
        .expect("List fund account validations should succeed");

    assert_eq!(list.count, 1);
    assert_eq!(list.items[0].id, "fav_00000000000001");
}
