use razorpay::{
    Creatable, Fetchable, RazorpayClientBuilder, Updatable,
    models::{
        Addon, CreateAddonRequest, CreatePlanItem, CreateSubscriptionRequest, PlanItem,
        Subscription, SubscriptionStatus, UpdateSubscriptionRequest,
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
async fn test_subscriptions_lifecycle() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server.uri()).await;

    let create_req = CreateSubscriptionRequest {
        plan_id: "plan_123".to_string(),
        total_count: 12,
        quantity: Some(1),
        start_at: None,
        expire_by: None,
        customer_notify: Some(true),
        addons: None,
        offer_id: None,
        notes: None,
        notify_info: None,
    };

    let expected_sub = Subscription {
        id: "sub_123".to_string(),
        entity: "subscription".to_string(),
        plan_id: "plan_123".to_string(),
        status: SubscriptionStatus::Created,
        current_start: None,
        current_end: None,
        ended_at: None,
        quantity: 1,
        notes: None,
        charge_at: None,
        start_at: None,
        end_at: None,
        auth_attempts: 0,
        total_count: 12,
        paid_count: 0,
        customer_notify: true,
        created_at: 1600000000,
        expire_by: None,
        short_url: Some("https://rzp.io/i/sub123".to_string()),
        has_scheduled_changes: false,
        change_scheduled_at: None,
        source: None,
        payment_method: None,
        offer_id: None,
        remaining_count: 12,
    };

    // 1. Create Subscription
    Mock::given(method("POST"))
        .and(path("/subscriptions"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .and(body_json(&create_req))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_sub))
        .mount(&mock_server)
        .await;

    let sub = client
        .subscriptions()
        .create(create_req, None)
        .await
        .expect("Create subscription should succeed");
    assert_eq!(sub.id, "sub_123");
    assert_eq!(sub.status, SubscriptionStatus::Created);

    // 2. Fetch Subscription
    Mock::given(method("GET"))
        .and(path("/subscriptions/sub_123"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_sub))
        .mount(&mock_server)
        .await;

    let fetched = client
        .subscriptions()
        .fetch("sub_123", None)
        .await
        .expect("Fetch subscription should succeed");
    assert_eq!(fetched.id, "sub_123");

    // 3. Update Subscription
    let update_req = UpdateSubscriptionRequest {
        plan_id: Some("plan_456".to_string()),
        quantity: Some(2),
        remaining_count: None,
        start_at: None,
        schedule_change_at: None,
        customer_notify: None,
        offer_id: None,
        notes: None,
    };

    let mut updated_sub = expected_sub.clone();
    updated_sub.plan_id = "plan_456".to_string();
    updated_sub.quantity = 2;

    Mock::given(method("PATCH"))
        .and(path("/subscriptions/sub_123"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .and(body_json(&update_req))
        .respond_with(ResponseTemplate::new(200).set_body_json(&updated_sub))
        .mount(&mock_server)
        .await;

    let updated = client
        .subscriptions()
        .update("sub_123", update_req, None)
        .await
        .expect("Update subscription should succeed");
    assert_eq!(updated.plan_id, "plan_456");
    assert_eq!(updated.quantity, 2);

    // 4. Create Addon on Subscription
    let addon_req = CreateAddonRequest {
        item: CreatePlanItem {
            name: "Extra Storage".to_string(),
            amount: 5000,
            currency: "INR".to_string(),
            description: None,
        },
        quantity: Some(1),
    };

    let expected_addon = Addon {
        id: "ao_123".to_string(),
        entity: "addon".to_string(),
        item: PlanItem {
            id: "item_ao1".to_string(),
            active: true,
            amount: 5000,
            unit_amount: 5000,
            currency: "INR".to_string(),
            name: "Extra Storage".to_string(),
            description: None,
        },
        subscription_id: Some("sub_123".to_string()),
        invoice_id: None,
        created_at: 1600000000,
    };

    Mock::given(method("POST"))
        .and(path("/subscriptions/sub_123/addons"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .and(body_json(&addon_req))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_addon))
        .mount(&mock_server)
        .await;

    let addon = client
        .subscriptions()
        .create_addon("sub_123", addon_req, None)
        .await
        .expect("Create addon on subscription should succeed");
    assert_eq!(addon.id, "ao_123");

    // 5. Pause Subscription
    let mut paused_sub = expected_sub.clone();
    paused_sub.status = SubscriptionStatus::Paused;

    Mock::given(method("POST"))
        .and(path("/subscriptions/sub_123/pause"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&paused_sub))
        .mount(&mock_server)
        .await;

    let paused = client
        .subscriptions()
        .pause("sub_123", Some("now"), None)
        .await
        .expect("Pause subscription should succeed");
    assert_eq!(paused.status, SubscriptionStatus::Paused);

    // 6. Resume Subscription
    let mut active_sub = expected_sub.clone();
    active_sub.status = SubscriptionStatus::Active;

    Mock::given(method("POST"))
        .and(path("/subscriptions/sub_123/resume"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&active_sub))
        .mount(&mock_server)
        .await;

    let resumed = client
        .subscriptions()
        .resume("sub_123", Some("now"), None)
        .await
        .expect("Resume subscription should succeed");
    assert_eq!(resumed.status, SubscriptionStatus::Active);

    // 7. Cancel Subscription
    let mut cancelled_sub = expected_sub.clone();
    cancelled_sub.status = SubscriptionStatus::Cancelled;

    Mock::given(method("POST"))
        .and(path("/subscriptions/sub_123/cancel"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&cancelled_sub))
        .mount(&mock_server)
        .await;

    let cancelled = client
        .subscriptions()
        .cancel("sub_123", false, None)
        .await
        .expect("Cancel subscription should succeed");
    assert_eq!(cancelled.status, SubscriptionStatus::Cancelled);
}
