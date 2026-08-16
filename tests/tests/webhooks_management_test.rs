use razorpay::{
    RazorpayClientBuilder,
    models::{CreateWebhookRequest, DeleteResponse, RazorpayList, UpdateWebhookRequest, Webhook},
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
async fn test_webhooks_management_crud() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server.uri()).await;

    let create_req = CreateWebhookRequest {
        url: "https://example.com/razorpay/webhook".to_string(),
        events: vec!["payment.captured".to_string(), "order.paid".to_string()],
        secret: Some("whsec_12345".to_string()),
        alert_email: Some("ops@example.com".to_string()),
        active: Some(true),
    };

    let expected_hook = Webhook {
        id: "hook_1234567890".to_string(),
        entity: Some("webhook".to_string()),
        url: "https://example.com/razorpay/webhook".to_string(),
        alert_email: Some("ops@example.com".to_string()),
        secret: Some("whsec_12345".to_string()),
        events: vec!["payment.captured".to_string(), "order.paid".to_string()],
        active: true,
        account_id: None,
        created_at: 1600000000,
        updated_at: None,
    };

    // 1. Create Webhook (v1)
    Mock::given(method("POST"))
        .and(path("/webhooks"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .and(body_json(&create_req))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_hook))
        .mount(&mock_server)
        .await;

    let hook = client
        .webhooks()
        .create(None, &create_req, None)
        .await
        .expect("Create webhook should succeed");
    assert_eq!(hook.id, "hook_1234567890");
    assert_eq!(hook.url, "https://example.com/razorpay/webhook");

    // 2. Fetch Webhook (v1)
    Mock::given(method("GET"))
        .and(path("/webhooks/hook_1234567890"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_hook))
        .mount(&mock_server)
        .await;

    let fetched = client
        .webhooks()
        .fetch("hook_1234567890", None, None)
        .await
        .expect("Fetch webhook should succeed");
    assert_eq!(fetched.id, "hook_1234567890");

    // 3. Edit Webhook (v1)
    let edit_req = UpdateWebhookRequest {
        url: Some("https://example.com/razorpay/webhook-v2".to_string()),
        events: None,
        secret: None,
        alert_email: None,
        active: None,
    };

    let mut updated_hook = expected_hook.clone();
    updated_hook.url = "https://example.com/razorpay/webhook-v2".to_string();

    Mock::given(method("PUT"))
        .and(path("/webhooks/hook_1234567890"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .and(body_json(&edit_req))
        .respond_with(ResponseTemplate::new(200).set_body_json(&updated_hook))
        .mount(&mock_server)
        .await;

    let edited = client
        .webhooks()
        .edit("hook_1234567890", None, &edit_req, None)
        .await
        .expect("Edit webhook should succeed");
    assert_eq!(edited.url, "https://example.com/razorpay/webhook-v2");

    // 4. List Webhooks (v1)
    let list_resp = RazorpayList {
        entity: "collection".to_string(),
        count: 1,
        items: vec![expected_hook.clone()],
    };

    Mock::given(method("GET"))
        .and(path("/webhooks"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&list_resp))
        .mount(&mock_server)
        .await;

    let list = client
        .webhooks()
        .all(None, None, None)
        .await
        .expect("List webhooks should succeed");
    assert_eq!(list.count, 1);
    assert_eq!(list.items[0].id, "hook_1234567890");

    // 5. Delete Webhook (v1)
    Mock::given(method("DELETE"))
        .and(path("/webhooks/hook_1234567890"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(DeleteResponse { deleted: true }))
        .mount(&mock_server)
        .await;

    let del = client
        .webhooks()
        .delete("hook_1234567890", None, None)
        .await
        .expect("Delete webhook should succeed");
    assert!(del.deleted);
}

#[tokio::test]
async fn test_webhooks_management_v2_account_scoped() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server.uri()).await;

    let create_req = CreateWebhookRequest {
        url: "https://partner.com/webhook".to_string(),
        events: vec!["payment.authorized".to_string()],
        secret: Some("whsec_route_123".to_string()),
        alert_email: None,
        active: Some(true),
    };

    let expected_hook = Webhook {
        id: "hook_route_999".to_string(),
        entity: Some("webhook".to_string()),
        url: "https://partner.com/webhook".to_string(),
        alert_email: None,
        secret: Some("whsec_route_123".to_string()),
        events: vec!["payment.authorized".to_string()],
        active: true,
        account_id: Some("acc_partner123".to_string()),
        created_at: 1600000000,
        updated_at: None,
    };

    // Create Route v2 Webhook
    Mock::given(method("POST"))
        .and(path("/v2/accounts/acc_partner123/webhooks"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .and(body_json(&create_req))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_hook))
        .mount(&mock_server)
        .await;

    let hook = client
        .webhooks()
        .create(Some("acc_partner123"), &create_req, None)
        .await
        .expect("Create v2 webhook should succeed");
    assert_eq!(hook.id, "hook_route_999");
    assert_eq!(hook.account_id.as_deref(), Some("acc_partner123"));
}
