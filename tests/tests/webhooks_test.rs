use razorpay::{
    error::RazorpayError,
    models::WebhookEventType,
    webhooks::{
        PaymentSignatureVerification, SubscriptionPaymentSignatureVerification,
        parse_webhook_event, verify_and_parse_webhook, verify_payment_signature,
        verify_subscription_payment_signature, verify_webhook_signature,
    },
};

#[test]
fn test_verify_webhook_signature_valid() {
    let payload = r#"{"entity":"event","event":"payment.captured"}"#;
    let secret = "secret123";

    use hmac::{Hmac, Mac};
    use sha2::Sha256;
    type HmacSha256 = Hmac<Sha256>;

    let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
    mac.update(payload.as_bytes());
    let valid_sig = hex::encode(mac.finalize().into_bytes());

    let result = verify_webhook_signature(payload, &valid_sig, secret);
    assert!(result.is_ok());
}

#[test]
fn test_verify_webhook_signature_tampered() {
    let payload = r#"{"entity":"event","event":"payment.captured"}"#;
    let secret = "secret123";
    let fake_sig = "a1b2c3d4e5f678901234567890abcdef1234567890abcdef1234567890abcdef";

    let result = verify_webhook_signature(payload, fake_sig, secret);
    assert!(matches!(result, Err(RazorpayError::SignatureMismatch)));
}

#[test]
fn test_verify_payment_signature_valid() {
    let order_id = "order_IEfcDis90VlTgP";
    let payment_id = "pay_IH4NVgfzrQI50M";
    let secret = "EnLs volumetric secret";

    use hmac::{Hmac, Mac};
    use sha2::Sha256;
    type HmacSha256 = Hmac<Sha256>;

    let data = format!("{}|{}", order_id, payment_id);
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
    mac.update(data.as_bytes());
    let valid_sig = hex::encode(mac.finalize().into_bytes());

    let result = verify_payment_signature(order_id, payment_id, &valid_sig, secret);
    assert!(result.is_ok());

    // Also test helper struct
    let verifier = PaymentSignatureVerification {
        order_id,
        payment_id,
        signature: &valid_sig,
        secret,
    };
    assert!(verifier.verify().is_ok());
}

#[test]
fn test_verify_subscription_payment_signature_valid() {
    let sub_id = "sub_00000000000001";
    let payment_id = "pay_00000000000001";
    let secret = "test_secret";

    use hmac::{Hmac, Mac};
    use sha2::Sha256;
    type HmacSha256 = Hmac<Sha256>;

    // HMAC payload format is: payment_id | subscription_id
    let data = format!("{}|{}", payment_id, sub_id);
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
    mac.update(data.as_bytes());
    let valid_sig = hex::encode(mac.finalize().into_bytes());

    // Parameters follow payload order: (payment_id, subscription_id)
    let result = verify_subscription_payment_signature(payment_id, sub_id, &valid_sig, secret);
    assert!(result.is_ok());

    // Also test helper struct
    let verifier = SubscriptionPaymentSignatureVerification {
        payment_id,
        subscription_id: sub_id,
        signature: &valid_sig,
        secret,
    };
    assert!(verifier.verify().is_ok());
}

#[test]
fn test_parse_webhook_event_payload() {
    let payload = r#"{
        "entity": "event",
        "account_id": "acc_12345",
        "event": "payment.captured",
        "contains": ["payment"],
        "payload": {
            "payment": {
                "entity": {
                    "id": "pay_test123",
                    "entity": "payment",
                    "amount": 50000,
                    "currency": "INR",
                    "status": "captured",
                    "order_id": "order_test123",
                    "invoice_id": null,
                    "international": false,
                    "method": "card",
                    "amount_refunded": 0,
                    "refund_status": null,
                    "captured": true,
                    "description": "Test Transaction",
                    "card_id": null,
                    "bank": null,
                    "wallet": null,
                    "vpa": null,
                    "email": "customer@example.com",
                    "contact": "+919876543210",
                    "notes": null,
                    "fee": 1000,
                    "tax": 180,
                    "error_code": null,
                    "error_description": null,
                    "error_source": null,
                    "error_step": null,
                    "error_reason": null,
                    "acquirer_data": null,
                    "created_at": 1600000000
                }
            }
        },
        "created_at": 1600000000
    }"#;

    let event = parse_webhook_event(payload).expect("Should parse webhook payload");
    assert_eq!(event.event, "payment.captured");
    assert_eq!(event.account_id, "acc_12345");

    let payment = event
        .payload
        .payment
        .expect("Payment entity should be present");
    assert_eq!(payment.entity.id, "pay_test123");
    assert_eq!(payment.entity.amount, 50000);
}

#[test]
fn test_verify_and_parse_webhook_combined() {
    let payload = r#"{"entity":"event","account_id":"acc_123","event":"payment.captured","contains":["payment"],"payload":{"payment":{"entity":{"id":"pay_123","entity":"payment","amount":5000,"currency":"INR","status":"captured","order_id":"order_123","invoice_id":null,"international":false,"method":"card","amount_refunded":0,"refund_status":null,"captured":true,"description":"test","card_id":null,"bank":null,"wallet":null,"vpa":null,"email":"test@example.com","contact":"+919999999999","notes":null,"fee":100,"tax":18,"error_code":null,"error_description":null,"error_source":null,"error_step":null,"error_reason":null,"acquirer_data":null,"created_at":1600000000}}},"created_at":1600000000}"#;
    let secret = "test_webhook_secret";

    use hmac::{Hmac, Mac};
    use sha2::Sha256;
    type HmacSha256 = Hmac<Sha256>;

    let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
    mac.update(payload.as_bytes());
    let sig = hex::encode(mac.finalize().into_bytes());

    let parsed = verify_and_parse_webhook(payload, &sig, secret)
        .expect("Combined verify and parse should succeed");

    assert_eq!(parsed.event, "payment.captured");
    assert_eq!(parsed.account_id, "acc_123");
    let event_type: WebhookEventType =
        serde_json::from_str(&format!("\"{}\"", parsed.event)).unwrap();
    assert_eq!(event_type, WebhookEventType::PaymentCaptured);
}

#[test]
fn test_verify_payment_link_signature_valid() {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;
    type HmacSha256 = Hmac<Sha256>;

    let payment_link_id = "plink_123456";
    let payment_link_reference_id = "ref_999";
    let payment_link_status = "paid";
    let payment_id = "pay_987654";
    let secret = "secret_xyz";

    let payload = format!(
        "{}|{}|{}|{}",
        payment_link_id, payment_link_reference_id, payment_link_status, payment_id
    );

    let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
    mac.update(payload.as_bytes());
    let sig = hex::encode(mac.finalize().into_bytes());

    // 1. Function test
    razorpay::webhooks::verify_payment_link_signature(
        payment_link_id,
        payment_link_reference_id,
        payment_link_status,
        payment_id,
        &sig,
        secret,
    )
    .expect("Payment link signature should verify");

    // 2. Helper struct test
    let verifier = razorpay::webhooks::PaymentLinkSignatureVerification {
        payment_link_id,
        payment_link_reference_id,
        payment_link_status,
        payment_id,
        signature: &sig,
        secret,
    };
    verifier
        .verify()
        .expect("Verifier helper struct should succeed");
}

#[test]
fn test_signature_verifier_debug_redaction() {
    let secret = "very_confidential_webhook_or_api_secret_123";

    let p_verifier = razorpay::webhooks::PaymentSignatureVerification {
        order_id: "order_123",
        payment_id: "pay_123",
        signature: "sig_123",
        secret,
    };
    let debug_str = format!("{:?}", p_verifier);
    assert!(!debug_str.contains(secret));
    assert!(debug_str.contains("[REDACTED]"));

    let s_verifier = razorpay::webhooks::SubscriptionPaymentSignatureVerification {
        payment_id: "pay_123",
        subscription_id: "sub_123",
        signature: "sig_123",
        secret,
    };
    let debug_str = format!("{:?}", s_verifier);
    assert!(!debug_str.contains(secret));
    assert!(debug_str.contains("[REDACTED]"));

    let pl_verifier = razorpay::webhooks::PaymentLinkSignatureVerification {
        payment_link_id: "plink_123",
        payment_link_reference_id: "ref_123",
        payment_link_status: "paid",
        payment_id: "pay_123",
        signature: "sig_123",
        secret,
    };
    let debug_str = format!("{:?}", pl_verifier);
    assert!(!debug_str.contains(secret));
    assert!(debug_str.contains("[REDACTED]"));
}
