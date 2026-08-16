use razorpay::{
    Creatable, Deletable, Fetchable, Listable, RazorpayClientBuilder, Updatable,
    models::{CreateItemRequest, DeleteResponse, Item, RazorpayList, UpdateItemRequest},
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
async fn test_items_crud_operations() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server.uri()).await;

    let create_req = CreateItemRequest {
        name: "Acme Widget".to_string(),
        amount: 29900,
        currency: "INR".to_string(),
        description: Some("High quality widget".to_string()),
    };

    let expected_item = Item {
        id: "item_123".to_string(),
        entity: Some("item".to_string()),
        active: true,
        amount: 29900,
        unit_amount: 29900,
        currency: "INR".to_string(),
        name: "Acme Widget".to_string(),
        description: Some("High quality widget".to_string()),
        unit: None,
        tax_inclusive: false,
        hsn_code: None,
        sac_code: None,
        tax_rate: None,
        taxes: None,
    };

    // 1. Create Item
    Mock::given(method("POST"))
        .and(path("/items"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .and(body_json(&create_req))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_item))
        .mount(&mock_server)
        .await;

    let item = client
        .items()
        .create(create_req, None)
        .await
        .expect("Create item should succeed");
    assert_eq!(item.id, "item_123");
    assert_eq!(item.name, "Acme Widget");

    // 2. Fetch Item
    Mock::given(method("GET"))
        .and(path("/items/item_123"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_item))
        .mount(&mock_server)
        .await;

    let fetched = client
        .items()
        .fetch("item_123", None)
        .await
        .expect("Fetch item should succeed");
    assert_eq!(fetched.id, "item_123");

    // 3. List Items
    let list_response = RazorpayList {
        entity: "collection".to_string(),
        count: 1,
        items: vec![expected_item.clone()],
    };

    Mock::given(method("GET"))
        .and(path("/items"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&list_response))
        .mount(&mock_server)
        .await;

    let list = client
        .items()
        .all(None, None)
        .await
        .expect("List items should succeed");
    assert_eq!(list.count, 1);
    assert_eq!(list.items[0].id, "item_123");

    // 4. Update Item
    let update_req = UpdateItemRequest {
        name: Some("Acme Widget Pro".to_string()),
        amount: Some(39900),
        currency: None,
        description: None,
        active: None,
    };

    let mut updated_item = expected_item;
    updated_item.name = "Acme Widget Pro".to_string();
    updated_item.amount = 39900;

    Mock::given(method("PATCH"))
        .and(path("/items/item_123"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .and(body_json(&update_req))
        .respond_with(ResponseTemplate::new(200).set_body_json(&updated_item))
        .mount(&mock_server)
        .await;

    let updated = client
        .items()
        .update("item_123", update_req, None)
        .await
        .expect("Update item should succeed");
    assert_eq!(updated.name, "Acme Widget Pro");
    assert_eq!(updated.amount, 39900);

    // 5. Delete Item
    Mock::given(method("DELETE"))
        .and(path("/items/item_123"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(DeleteResponse { deleted: true }))
        .mount(&mock_server)
        .await;

    let del_resp = client
        .items()
        .delete("item_123", None)
        .await
        .expect("Delete item should succeed");
    assert!(del_resp.deleted);
}
