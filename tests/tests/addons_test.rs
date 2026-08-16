use razorpay::{
    Deletable, Fetchable, Listable, RazorpayClientBuilder,
    models::{Addon, DeleteResponse, PlanItem, RazorpayList},
};
use std::time::Duration;
use url::Url;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{basic_auth, method, path},
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
async fn test_addons_operations() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server.uri()).await;

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

    // 1. Fetch Addon
    Mock::given(method("GET"))
        .and(path("/addons/ao_123"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_addon))
        .mount(&mock_server)
        .await;

    let fetched = client
        .addons()
        .fetch("ao_123", None)
        .await
        .expect("Fetch addon should succeed");
    assert_eq!(fetched.id, "ao_123");

    // 2. List Addons
    let list_response = RazorpayList {
        entity: "collection".to_string(),
        count: 1,
        items: vec![expected_addon],
    };

    Mock::given(method("GET"))
        .and(path("/addons"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&list_response))
        .mount(&mock_server)
        .await;

    let list = client
        .addons()
        .all(None, None)
        .await
        .expect("List addons should succeed");
    assert_eq!(list.count, 1);
    assert_eq!(list.items[0].id, "ao_123");

    // 3. Delete Addon
    Mock::given(method("DELETE"))
        .and(path("/addons/ao_123"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(DeleteResponse { deleted: true }))
        .mount(&mock_server)
        .await;

    let del_resp = client
        .addons()
        .delete("ao_123", None)
        .await
        .expect("Delete addon should succeed");
    assert!(del_resp.deleted);
}
