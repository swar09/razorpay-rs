use razorpay::{
    models::{Card, Iin, ListOptions, RazorpayList},
    Fetchable, RazorpayClientBuilder,
};
use std::time::Duration;
use url::Url;
use wiremock::{
    matchers::{basic_auth, method, path},
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
async fn test_cards_operations() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server.uri()).await;

    let expected_card = Card {
        id: "card_123".to_string(),
        entity: "card".to_string(),
        name: Some("Gaurav Kumar".to_string()),
        last4: "4321".to_string(),
        network: "Visa".to_string(),
        card_type: Some("credit".to_string()),
        sub_type: Some("consumer".to_string()),
        issuer: Some("HDFC".to_string()),
        international: Some(false),
        emi: Some(true),
        expiry_month: Some(12),
        expiry_year: Some(2028),
    };

    // 1. Fetch Card by ID
    Mock::given(method("GET"))
        .and(path("/cards/card_123"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_card))
        .mount(&mock_server)
        .await;

    let card = client
        .cards()
        .fetch("card_123", None)
        .await
        .expect("Card fetch should succeed");
    assert_eq!(card.id, "card_123");
    assert_eq!(card.network, "Visa");

    // 2. Request Card Reference / Fingerprint
    Mock::given(method("POST"))
        .and(path("/cards/fingerprints"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!({"entity": "card_fingerprint", "fingerprint": "fp_abc123"})),
        )
        .mount(&mock_server)
        .await;

    let fp_resp = client
        .cards()
        .request_card_reference(&serde_json::json!({"token": "tok_123"}), None)
        .await
        .expect("Card fingerprint request should succeed");
    assert_eq!(fp_resp["fingerprint"], "fp_abc123");
}

#[tokio::test]
async fn test_iins_operations() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server.uri()).await;

    let expected_iin = Iin {
        iin: "411111".to_string(),
        entity: Some("iin".to_string()),
        network: Some("Visa".to_string()),
        card_type: Some("credit".to_string()),
        sub_type: Some("consumer".to_string()),
        issuer_code: Some("HDFC".to_string()),
        issuer_name: Some("HDFC Bank".to_string()),
        international: Some(false),
        is_tokenized: Some(false),
        recurring: Some(true),
    };

    // 1. Fetch IIN
    Mock::given(method("GET"))
        .and(path("/iins/411111"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_iin))
        .mount(&mock_server)
        .await;

    let iin = client
        .iins()
        .fetch("411111", None)
        .await
        .expect("IIN fetch should succeed");
    assert_eq!(iin.iin, "411111");
    assert_eq!(iin.network, Some("Visa".to_string()));

    // 2. Fetch all IINs list
    let list_response = RazorpayList {
        entity: "collection".to_string(),
        count: 1,
        items: vec![expected_iin],
    };

    Mock::given(method("GET"))
        .and(path("/iins/list"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&list_response))
        .mount(&mock_server)
        .await;

    let list = client
        .iins()
        .all(Some(ListOptions { count: Some(10), skip: None, from: None, to: None }), None)
        .await
        .expect("IINs list should succeed");
    assert_eq!(list.count, 1);
}
