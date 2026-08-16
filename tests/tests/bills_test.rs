use razorpay::{
    Creatable, Deletable, Fetchable, RazorpayClientBuilder, Updatable,
    models::{CreateBillRequest, UpdateBillRequest},
};
use wiremock::{
    Mock, ResponseTemplate,
    matchers::{method, path},
};

async fn create_test_client(server_uri: &str) -> razorpay::RazorpayClient {
    RazorpayClientBuilder::new()
        .key_id("rzp_test_key")
        .key_secret("test_secret")
        .base_url(url::Url::parse(server_uri).unwrap())
        .build()
        .unwrap()
}

#[tokio::test]
async fn test_bills_operations() {
    let mock_server = wiremock::MockServer::start().await;
    let client = create_test_client(&mock_server.uri()).await;

    // 1. Create Bill
    let bill_json = serde_json::json!({
        "id": "bill_PYamApGCFTAjkh",
        "business_type": "retail",
        "business_category": "retail_and_consumer_goods",
        "receipt_number": "INV001250010",
        "receipt_type": "tax_invoice",
        "receipt_delivery": "digital",
        "receipt_url": "yourbill.me/PYamApGCFTAjkh"
    });

    Mock::given(method("POST"))
        .and(path("/bills"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&bill_json))
        .mount(&mock_server)
        .await;

    let create_payload = CreateBillRequest {
        store_code: Some("JK-001".to_string()),
        business_type: Some("retail".to_string()),
        business_category: Some("retail_and_consumer_goods".to_string()),
        receipt_number: Some("INV001250010".to_string()),
        ..Default::default()
    };

    let created = client.bills().create(create_payload, None).await.unwrap();
    assert_eq!(created.id, "bill_PYamApGCFTAjkh");
    assert_eq!(created.business_type.as_deref(), Some("retail"));

    // 2. Fetch Bill
    Mock::given(method("GET"))
        .and(path("/bills/bill_PYamApGCFTAjkh"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&bill_json))
        .mount(&mock_server)
        .await;

    let fetched = client
        .bills()
        .fetch("bill_PYamApGCFTAjkh", None)
        .await
        .unwrap();
    assert_eq!(fetched.id, "bill_PYamApGCFTAjkh");

    // 3. Update Bill
    Mock::given(method("PATCH"))
        .and(path("/bills/bill_PYamApGCFTAjkh"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&bill_json))
        .mount(&mock_server)
        .await;

    let update_payload = UpdateBillRequest {
        customer: None,
        line_items: None,
        receipt_summary: None,
        taxes: None,
        payments: None,
        tags: Some(vec!["pos".to_string()]),
    };

    let updated = client
        .bills()
        .update("bill_PYamApGCFTAjkh", update_payload, None)
        .await
        .unwrap();
    assert_eq!(updated.id, "bill_PYamApGCFTAjkh");

    // 4. Delete Bill
    Mock::given(method("DELETE"))
        .and(path("/bills/bill_PYamApGCFTAjkh"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(serde_json::json!({ "deleted": true })),
        )
        .mount(&mock_server)
        .await;

    let deleted = client
        .bills()
        .delete("bill_PYamApGCFTAjkh", None)
        .await
        .unwrap();
    assert!(deleted.deleted);
}
