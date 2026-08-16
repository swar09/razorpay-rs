use razorpay::{
    models::{
        EditTransferRequest, RazorpayList, ReverseTransferRequest, Transfer, TransferRequest,
        TransferReversal,
    },
    Creatable, Fetchable, Listable, RazorpayClientBuilder, Updatable,
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
async fn test_transfers_operations() {
    let mock_server = MockServer::start().await;
    let client = create_test_client(&mock_server.uri()).await;

    let transfer_req = TransferRequest {
        account: "acc_12345".to_string(),
        amount: 25000,
        currency: "INR".to_string(),
        notes: None,
        linked_account_notes: None,
        on_hold: Some(false),
        on_hold_until: None,
    };

    let expected_transfer = Transfer {
        id: "trf_123".to_string(),
        entity: "transfer".to_string(),
        source: "pay_123".to_string(),
        recipient: "acc_12345".to_string(),
        amount: 25000,
        currency: "INR".to_string(),
        amount_reversed: 0,
        notes: None,
        linked_account_notes: None,
        on_hold: false,
        on_hold_until: None,
        recipient_settlement_id: None,
        created_at: 1600000000,
        processed_at: None,
        error: None,
    };

    // 1. Create Transfer
    Mock::given(method("POST"))
        .and(path("/transfers"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .and(body_json(&transfer_req))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_transfer))
        .mount(&mock_server)
        .await;

    let trf = client
        .transfers()
        .create(transfer_req, None)
        .await
        .expect("Create transfer should succeed");
    assert_eq!(trf.id, "trf_123");
    assert_eq!(trf.amount, 25000);

    // 2. Fetch Transfer
    Mock::given(method("GET"))
        .and(path("/transfers/trf_123"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_transfer))
        .mount(&mock_server)
        .await;

    let fetched = client
        .transfers()
        .fetch("trf_123", None)
        .await
        .expect("Fetch transfer should succeed");
    assert_eq!(fetched.id, "trf_123");

    // 3. Update Transfer
    let edit_req = EditTransferRequest {
        on_hold: true,
        on_hold_until: Some(1600003600),
    };

    let mut updated_transfer = expected_transfer.clone();
    updated_transfer.on_hold = true;
    updated_transfer.on_hold_until = Some(1600003600);

    Mock::given(method("PATCH"))
        .and(path("/transfers/trf_123"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .and(body_json(&edit_req))
        .respond_with(ResponseTemplate::new(200).set_body_json(&updated_transfer))
        .mount(&mock_server)
        .await;

    let updated = client
        .transfers()
        .update("trf_123", edit_req, None)
        .await
        .expect("Update transfer should succeed");
    assert!(updated.on_hold);

    // 4. Reverse Transfer
    let reverse_req = ReverseTransferRequest {
        amount: Some(10000),
        notes: None,
        reverse_all: None,
    };

    let expected_reversal = TransferReversal {
        id: "rev_123".to_string(),
        entity: "reversal".to_string(),
        transfer_id: "trf_123".to_string(),
        amount: 10000,
        fee: 100,
        tax: 18,
        currency: "INR".to_string(),
        notes: None,
        initiator_id: None,
        customer_refund_id: None,
        created_at: 1600000000,
    };

    Mock::given(method("POST"))
        .and(path("/transfers/trf_123/reversals"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .and(body_json(&reverse_req))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_reversal))
        .mount(&mock_server)
        .await;

    let reversal = client
        .transfers()
        .reverse("trf_123", reverse_req, None)
        .await
        .expect("Reverse transfer should succeed");
    assert_eq!(reversal.id, "rev_123");
    assert_eq!(reversal.amount, 10000);

    // 5. List All Transfers
    let list_response = RazorpayList {
        entity: "collection".to_string(),
        count: 1,
        items: vec![expected_transfer],
    };

    Mock::given(method("GET"))
        .and(path("/transfers"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&list_response))
        .mount(&mock_server)
        .await;

    let list = client
        .transfers()
        .all(None, None)
        .await
        .expect("List transfers should succeed");
    assert_eq!(list.count, 1);
    assert_eq!(list.items[0].id, "trf_123");

    // 6. Fetch Transfer Reversals
    let reversals_response = RazorpayList {
        entity: "collection".to_string(),
        count: 1,
        items: vec![expected_reversal],
    };

    Mock::given(method("GET"))
        .and(path("/transfers/trf_123/reversals"))
        .and(basic_auth("rzp_test_key", "test_secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&reversals_response))
        .mount(&mock_server)
        .await;

    let reversals = client
        .transfers()
        .reversals("trf_123", None, None)
        .await
        .expect("Fetch reversals should succeed");
    assert_eq!(reversals.count, 1);
    assert_eq!(reversals.items[0].id, "rev_123");
}
