use razorpay::{
    Creatable, Fetchable, Listable, RazorpayClientBuilder,
    models::{
        CreateVirtualAccountReceivers, CreateVirtualAccountRequest, RazorpayList, VirtualAccount,
        VirtualAccountStatus,
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
async fn test_virtual_accounts_operations() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server.uri()).await;

    let create_req = CreateVirtualAccountRequest {
        receivers: CreateVirtualAccountReceivers {
            types: vec!["bank_account".to_string()],
        },
        description: Some("Virtual Account for Order #100".to_string()),
        amount: Some(100000),
        customer_id: None,
        close_by: None,
        notes: None,
    };

    let expected_va = VirtualAccount {
        id: "va_123".to_string(),
        entity: "virtual_account".to_string(),
        name: "Acme Corp".to_string(),
        description: Some("Virtual Account for Order #100".to_string()),
        amount_expected: Some(100000),
        amount_paid: 0,
        status: VirtualAccountStatus::Active,
        receivers: None,
        close_by: None,
        closed_at: None,
        close_reason: None,
        notes: None,
        customer_id: None,
        created_at: 1600000000,
    };

    // 1. Create Virtual Account
    Mock::given(method("POST"))
        .and(path("/virtual_accounts"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .and(body_json(&create_req))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_va))
        .mount(&mock_server)
        .await;

    let va = client
        .virtual_accounts()
        .create(create_req, None)
        .await
        .expect("Create virtual account should succeed");
    assert_eq!(va.id, "va_123");
    assert_eq!(va.status, VirtualAccountStatus::Active);

    // 2. Fetch Virtual Account
    Mock::given(method("GET"))
        .and(path("/virtual_accounts/va_123"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_va))
        .mount(&mock_server)
        .await;

    let fetched = client
        .virtual_accounts()
        .fetch("va_123", None)
        .await
        .expect("Fetch virtual account should succeed");
    assert_eq!(fetched.id, "va_123");

    // 3. List Virtual Accounts
    let list_response = RazorpayList {
        entity: "collection".to_string(),
        count: 1,
        items: vec![expected_va.clone()],
    };

    Mock::given(method("GET"))
        .and(path("/virtual_accounts"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&list_response))
        .mount(&mock_server)
        .await;

    let list = client
        .virtual_accounts()
        .all(None, None)
        .await
        .expect("List virtual accounts should succeed");
    assert_eq!(list.count, 1);
    assert_eq!(list.items[0].id, "va_123");

    // 4. Close Virtual Account
    let mut closed_va = expected_va;
    closed_va.status = VirtualAccountStatus::Closed;

    Mock::given(method("POST"))
        .and(path("/virtual_accounts/va_123/close"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&closed_va))
        .mount(&mock_server)
        .await;

    let closed = client
        .virtual_accounts()
        .close("va_123", None)
        .await
        .expect("Close virtual account should succeed");
    assert_eq!(closed.status, VirtualAccountStatus::Closed);

    // 5. Delete Receiver
    Mock::given(method("DELETE"))
        .and(path("/virtual_accounts/va_123/receivers/recv_456"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(serde_json::json!({"deleted": true})),
        )
        .mount(&mock_server)
        .await;

    let del_resp = client
        .virtual_accounts()
        .delete_receiver("va_123", "recv_456", None)
        .await
        .expect("Delete receiver should succeed");
    assert!(del_resp.deleted);
}
