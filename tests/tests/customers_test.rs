use razorpay::{
    models::{
        CardDetails, CreateCustomerRequest, Customer, DeleteResponse, EditCustomerRequest, Token,
    },
    Creatable, Deletable, Fetchable, RazorpayClientBuilder, Updatable,
};
use std::time::Duration;
use url::Url;
use wiremock::{
    matchers::{basic_auth, body_json, method, path},
    Mock, MockServer, ResponseTemplate,
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
async fn test_customers_crud_and_tokens() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server.uri()).await;

    let create_req = CreateCustomerRequest {
        name: "Gaurav Kumar".to_string(),
        email: Some("gaurav@example.com".to_string()),
        contact: Some("+919876543210".to_string()),
        gstin: None,
        fail_existing: Some(0),
        notes: None,
    };

    let expected_customer = Customer {
        id: "cust_100".to_string(),
        entity: "customer".to_string(),
        name: Some("Gaurav Kumar".to_string()),
        email: Some("gaurav@example.com".to_string()),
        contact: Some("+919876543210".to_string()),
        gstin: None,
        notes: None,
        created_at: 1600000000,
    };

    // 1. Create Customer
    Mock::given(method("POST"))
        .and(path("/customers"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .and(body_json(&create_req))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_customer))
        .mount(&mock_server)
        .await;

    let customer = client
        .customers()
        .create(create_req, None)
        .await
        .expect("Create customer should succeed");
    assert_eq!(customer.id, "cust_100");

    // 2. Fetch Customer
    Mock::given(method("GET"))
        .and(path("/customers/cust_100"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_customer))
        .mount(&mock_server)
        .await;

    let fetched = client
        .customers()
        .fetch("cust_100", None)
        .await
        .expect("Fetch customer should succeed");
    assert_eq!(fetched.id, "cust_100");

    // 3. Edit Customer
    let edit_req = EditCustomerRequest {
        name: Some("Gaurav K.".to_string()),
        email: None,
        contact: None,
        gstin: None,
        notes: None,
    };

    let mut updated_customer = expected_customer.clone();
    updated_customer.name = Some("Gaurav K.".to_string());

    Mock::given(method("PUT"))
        .and(path("/customers/cust_100"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .and(body_json(&edit_req))
        .respond_with(ResponseTemplate::new(200).set_body_json(&updated_customer))
        .mount(&mock_server)
        .await;

    let updated = client
        .customers()
        .update("cust_100", edit_req, None)
        .await
        .expect("Edit customer should succeed");
    assert_eq!(updated.name, Some("Gaurav K.".to_string()));

    // 4. Nested Customer Tokens
    let expected_token = Token {
        id: "token_123".to_string(),
        entity: "token".to_string(),
        customer_id: Some("cust_100".to_string()),
        token: Some("tok_abc".to_string()),
        method: Some("card".to_string()),
        card: Some(CardDetails {
            id: "card_99".to_string(),
            last4: "4321".to_string(),
            network: "MasterCard".to_string(),
            ..Default::default()
        }),
        bank: None,
        wallet: None,
        vpa: None,
        recurring: Some(true),
        auth_type: None,
        max_amount: None,
        status: Some("confirmed".to_string()),
        created_at: 1600000000,
    };

    Mock::given(method("GET"))
        .and(path("/customers/cust_100/tokens/token_123"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_token))
        .mount(&mock_server)
        .await;

    let token = client
        .customers()
        .tokens("cust_100")
        .fetch("token_123", None)
        .await
        .expect("Fetch customer token should succeed");
    assert_eq!(token.id, "token_123");

    // 5. Delete Customer Token
    Mock::given(method("DELETE"))
        .and(path("/customers/cust_100/tokens/token_123"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(DeleteResponse { deleted: true }))
        .mount(&mock_server)
        .await;

    let del_resp = client
        .customers()
        .tokens("cust_100")
        .delete("token_123", None)
        .await
        .expect("Delete token should succeed");
    assert!(del_resp.deleted);
}
