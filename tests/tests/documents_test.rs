use razorpay::{Fetchable, RazorpayClientBuilder, models::Document};
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
async fn test_documents_fetch() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server.uri()).await;

    let expected_doc = Document {
        id: "doc_123".to_string(),
        entity: "document".to_string(),
        name: "kyc_pan.pdf".to_string(),
        document_type: "identity_proof".to_string(),
        document_category: Some("individual".to_string()),
        url: Some("https://rzp.io/docs/doc_123.pdf".to_string()),
        size: Some(102400),
        created_at: 1600000000,
    };

    Mock::given(method("GET"))
        .and(path("/documents/doc_123"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_doc))
        .mount(&mock_server)
        .await;

    let doc = client
        .documents()
        .fetch("doc_123", None)
        .await
        .expect("Fetch document should succeed");
    assert_eq!(doc.id, "doc_123");
    assert_eq!(doc.name, "kyc_pan.pdf");
}
