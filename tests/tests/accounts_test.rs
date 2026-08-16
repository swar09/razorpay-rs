use razorpay::{
    Deletable, Fetchable, Listable, RazorpayClientBuilder,
    models::{DeleteResponse, LinkedAccount, Stakeholder},
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
async fn test_accounts_and_stakeholders_operations() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server.uri()).await;

    let create_account_body = serde_json::json!({
        "email": "merchant@example.com",
        "type": "standard"
    });

    let expected_account = LinkedAccount {
        id: "acc_999".to_string(),
        entity: "account".to_string(),
        type_: Some("standard".to_string()),
        status: Some("created".to_string()),
        email: "merchant@example.com".to_string(),
        profile: None,
        notes: None,
        created_at: 1600000000,
    };

    // 1. Create Linked Account (v2)
    Mock::given(method("POST"))
        .and(path("/v2/accounts"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .and(body_json(&create_account_body))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_account))
        .mount(&mock_server)
        .await;

    let account = client
        .accounts()
        .create(&create_account_body, None)
        .await
        .expect("Create account should succeed");
    assert_eq!(account.id, "acc_999");
    assert_eq!(account.email, "merchant@example.com");

    // 2. Fetch Linked Account (v2)
    Mock::given(method("GET"))
        .and(path("/v2/accounts/acc_999"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_account))
        .mount(&mock_server)
        .await;

    let fetched = client
        .accounts()
        .fetch("acc_999", None)
        .await
        .expect("Fetch account should succeed");
    assert_eq!(fetched.id, "acc_999");

    // 3. Delete Linked Account (v2)
    Mock::given(method("DELETE"))
        .and(path("/v2/accounts/acc_999"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(DeleteResponse { deleted: true }))
        .mount(&mock_server)
        .await;

    let del_resp = client
        .accounts()
        .delete("acc_999", None)
        .await
        .expect("Delete account should succeed");
    assert!(del_resp.deleted);

    // 4. Create Stakeholder (v2)
    let stakeholder_body = serde_json::json!({
        "name": "Jane Doe",
        "email": "jane@example.com"
    });

    let expected_stakeholder = Stakeholder {
        id: "sth_123".to_string(),
        entity: "stakeholder".to_string(),
        name: Some("Jane Doe".to_string()),
        email: Some("jane@example.com".to_string()),
        phone: None,
        relationship: None,
        notes: None,
        created_at: 1600000000,
    };

    Mock::given(method("POST"))
        .and(path("/v2/accounts/acc_999/stakeholders"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .and(body_json(&stakeholder_body))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_stakeholder))
        .mount(&mock_server)
        .await;

    let sth = client
        .accounts()
        .stakeholders("acc_999")
        .create(&stakeholder_body, None)
        .await
        .expect("Create stakeholder should succeed");
    assert_eq!(sth.id, "sth_123");

    // 5. Fetch Stakeholder (v2)
    Mock::given(method("GET"))
        .and(path("/v2/accounts/acc_999/stakeholders/sth_123"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_stakeholder))
        .mount(&mock_server)
        .await;

    let fetched_sth = client
        .accounts()
        .stakeholders("acc_999")
        .fetch("sth_123", None)
        .await
        .expect("Fetch stakeholder should succeed");
    assert_eq!(fetched_sth.id, "sth_123");

    // 6. List Stakeholders (v2)
    let list_response = razorpay::models::RazorpayList {
        entity: "collection".to_string(),
        count: 1,
        items: vec![expected_stakeholder],
    };

    Mock::given(method("GET"))
        .and(path("/v2/accounts/acc_999/stakeholders"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&list_response))
        .mount(&mock_server)
        .await;

    let list = client
        .accounts()
        .stakeholders("acc_999")
        .all(None, None)
        .await
        .expect("List stakeholders should succeed");
    assert_eq!(list.count, 1);
    assert_eq!(list.items[0].id, "sth_123");
}
