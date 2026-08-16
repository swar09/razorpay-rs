use razorpay::{
    error::RazorpayError,
    webhooks::{
        verify_payment_signature, verify_subscription_payment_signature, verify_webhook_signature,
    },
};

#[test]
fn test_verify_webhook_signature_valid() {
    let payload = r#"{"entity":"event","event":"payment.captured"}"#;
    let secret = "secret123";

    // Known HMAC-SHA256 for payload with secret123:
    // echo -n '{"entity":"event","event":"payment.captured"}' | openssl dgst -sha256 -hmac 'secret123'
    // Let's compute via hmac or verify using standard algorithm
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
}

#[test]
fn test_verify_subscription_payment_signature_valid() {
    let sub_id = "sub_00000000000001";
    let payment_id = "pay_00000000000001";
    let secret = "test_secret";

    use hmac::{Hmac, Mac};
    use sha2::Sha256;
    type HmacSha256 = Hmac<Sha256>;

    let data = format!("{}|{}", payment_id, sub_id);
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
    mac.update(data.as_bytes());
    let valid_sig = hex::encode(mac.finalize().into_bytes());

    let result = verify_subscription_payment_signature(sub_id, payment_id, &valid_sig, secret);
    assert!(result.is_ok());
}
