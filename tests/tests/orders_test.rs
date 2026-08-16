use razorpay::{
    Creatable, Fetchable, RazorpayClientBuilder,
    error::RazorpayError,
    models::{CreateOrderRequest, Order, OrderStatus},
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
async fn test_orders_create_success() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server.uri()).await;

    let request_data = CreateOrderRequest {
        amount: 50000,
        currency: "INR".to_string(),
        receipt: Some("rcpt_101".to_string()),
        partial_payment: None,
        first_payment_min_amount: None,
        transfers: None,
        notes: None,
    };

    let expected_order = Order {
        id: "order_Hk1234567890".to_string(),
        entity: "order".to_string(),
        amount: 50000,
        amount_paid: Some(0),
        amount_due: Some(50000),
        currency: "INR".to_string(),
        receipt: Some("rcpt_101".to_string()),
        offer_id: None,
        status: OrderStatus::Created,
        partial_payment: false,
        attempts: 0,
        notes: None,
        created_at: 1600000000,
    };

    Mock::given(method("POST"))
        .and(path("/orders"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .and(body_json(&request_data))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_order))
        .mount(&mock_server)
        .await;

    let order = client
        .orders()
        .create(request_data, None)
        .await
        .expect("Order creation should succeed");

    assert_eq!(order.id, "order_Hk1234567890");
    assert_eq!(order.amount, 50000);
    assert_eq!(order.status, OrderStatus::Created);
}

#[tokio::test]
async fn test_orders_fetch_success() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server.uri()).await;

    let expected_order = Order {
        id: "order_Hk1234567890".to_string(),
        entity: "order".to_string(),
        amount: 50000,
        amount_paid: Some(50000),
        amount_due: Some(0),
        currency: "INR".to_string(),
        receipt: Some("rcpt_101".to_string()),
        offer_id: None,
        status: OrderStatus::Paid,
        partial_payment: false,
        attempts: 1,
        notes: None,
        created_at: 1600000000,
    };

    Mock::given(method("GET"))
        .and(path("/orders/order_Hk1234567890"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_order))
        .mount(&mock_server)
        .await;

    let order = client
        .orders()
        .fetch("order_Hk1234567890", None)
        .await
        .expect("Order fetch should succeed");

    assert_eq!(order.id, "order_Hk1234567890");
    assert_eq!(order.status, OrderStatus::Paid);
}

#[tokio::test]
async fn test_orders_create_api_error_response() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server.uri()).await;

    let request_data = CreateOrderRequest {
        amount: 100,
        currency: "INVALID_CURRENCY".to_string(),
        receipt: None,
        partial_payment: None,
        first_payment_min_amount: None,
        transfers: None,
        notes: None,
    };

    let error_body = serde_json::json!({
        "error": {
            "code": "BAD_REQUEST_ERROR",
            "description": "currency is not supported",
            "field": "currency",
            "source": "business",
            "step": "payment_initiation",
            "reason": "input_validation_failed"
        }
    });

    Mock::given(method("POST"))
        .and(path("/orders"))
        .respond_with(ResponseTemplate::new(400).set_body_json(error_body))
        .mount(&mock_server)
        .await;

    let result = client.orders().create(request_data, None).await;

    match result {
        Err(RazorpayError::Api(api_err)) => {
            assert_eq!(api_err.code, "BAD_REQUEST_ERROR");
            assert_eq!(api_err.description, "currency is not supported");
            assert_eq!(api_err.field, Some("currency".to_string()));
        }
        other => panic!("Expected RazorpayError::Api, got {:?}", other),
    }
}
