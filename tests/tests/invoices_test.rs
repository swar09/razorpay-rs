use razorpay::{
    Creatable, Deletable, RazorpayClientBuilder,
    models::{CreateInvoiceRequest, DeleteResponse, Invoice, InvoiceStatus},
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
async fn test_invoices_operations() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server.uri()).await;

    let create_inv_req = CreateInvoiceRequest {
        invoice_type: Some("invoice".to_string()),
        description: Some("Service Invoice".to_string()),
        customer_id: Some("cust_100".to_string()),
        customer: None,
        line_items: None,
        expire_by: None,
        sms_notify: Some(true),
        email_notify: Some(true),
        partial_payment: Some(false),
        currency: Some("INR".to_string()),
        notes: None,
    };

    let expected_invoice = Invoice {
        id: "inv_123".to_string(),
        entity: "invoice".to_string(),
        invoice_type: "invoice".to_string(),
        status: InvoiceStatus::Draft,
        invoice_number: Some("INV-001".to_string()),
        customer_id: Some("cust_100".to_string()),
        customer_details: None,
        order_id: None,
        line_items: vec![],
        payment_id: None,
        date: None,
        due_date: None,
        expire_by: None,
        expired_at: None,
        issued_at: None,
        paid_at: None,
        cancelled_at: None,
        sms_status: None,
        email_status: None,
        currency: "INR".to_string(),
        amount: Some(5000),
        amount_paid: Some(0),
        amount_due: Some(5000),
        short_url: Some("https://rzp.io/i/inv123".to_string()),
        description: Some("Service Invoice".to_string()),
        notes: None,
        terms: None,
        comment: None,
        created_at: 1600000000,
    };

    // 1. Create Invoice
    Mock::given(method("POST"))
        .and(path("/invoices"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .and(body_json(&create_inv_req))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_invoice))
        .mount(&mock_server)
        .await;

    let inv = client
        .invoices()
        .create(create_inv_req, None)
        .await
        .expect("Create invoice should succeed");
    assert_eq!(inv.id, "inv_123");
    assert_eq!(inv.status, InvoiceStatus::Draft);

    // 2. Issue Draft Invoice
    let mut issued_invoice = expected_invoice.clone();
    issued_invoice.status = InvoiceStatus::Issued;
    issued_invoice.issued_at = Some(1600000100);

    Mock::given(method("POST"))
        .and(path("/invoices/inv_123/issue"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&issued_invoice))
        .mount(&mock_server)
        .await;

    let issued = client
        .invoices()
        .issue("inv_123", None)
        .await
        .expect("Issue invoice should succeed");
    assert_eq!(issued.status, InvoiceStatus::Issued);

    // 3. Delete Draft Invoice
    Mock::given(method("DELETE"))
        .and(path("/invoices/inv_123"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(DeleteResponse { deleted: true }))
        .mount(&mock_server)
        .await;

    let del_resp = client
        .invoices()
        .delete("inv_123", None)
        .await
        .expect("Delete draft invoice should succeed");
    assert!(del_resp.deleted);
}
