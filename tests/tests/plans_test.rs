use razorpay::{
    Creatable, Fetchable, Listable, RazorpayClientBuilder,
    models::{CreatePlanItem, CreatePlanRequest, Plan, PlanItem, PlanPeriod, RazorpayList},
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
async fn test_plans_operations() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server.uri()).await;

    let create_req = CreatePlanRequest {
        period: PlanPeriod::Monthly,
        interval: 1,
        item: CreatePlanItem {
            name: "Pro Monthly".to_string(),
            amount: 99900,
            currency: "INR".to_string(),
            description: Some("Monthly Pro subscription".to_string()),
        },
        notes: None,
    };

    let expected_plan = Plan {
        id: "plan_123".to_string(),
        entity: "plan".to_string(),
        interval: 1,
        period: PlanPeriod::Monthly,
        item: PlanItem {
            id: "item_123".to_string(),
            active: true,
            amount: 99900,
            unit_amount: 99900,
            currency: "INR".to_string(),
            name: "Pro Monthly".to_string(),
            description: Some("Monthly Pro subscription".to_string()),
        },
        notes: None,
        created_at: 1600000000,
    };

    // 1. Create Plan
    Mock::given(method("POST"))
        .and(path("/plans"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .and(body_json(&create_req))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_plan))
        .mount(&mock_server)
        .await;

    let plan = client
        .plans()
        .create(create_req, None)
        .await
        .expect("Create plan should succeed");
    assert_eq!(plan.id, "plan_123");
    assert_eq!(plan.period, PlanPeriod::Monthly);

    // 2. Fetch Plan
    Mock::given(method("GET"))
        .and(path("/plans/plan_123"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_plan))
        .mount(&mock_server)
        .await;

    let fetched = client
        .plans()
        .fetch("plan_123", None)
        .await
        .expect("Fetch plan should succeed");
    assert_eq!(fetched.id, "plan_123");

    // 3. List Plans
    let list_response = RazorpayList {
        entity: "collection".to_string(),
        count: 1,
        items: vec![expected_plan],
    };

    Mock::given(method("GET"))
        .and(path("/plans"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&list_response))
        .mount(&mock_server)
        .await;

    let list = client
        .plans()
        .all(None, None)
        .await
        .expect("List plans should succeed");
    assert_eq!(list.count, 1);
    assert_eq!(list.items[0].id, "plan_123");
}
