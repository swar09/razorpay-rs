use hmac::{Hmac, Mac};
use sha2::Sha256;

use crate::error::{RazorpayError, RazorpayResult};

type HmacSha256 = Hmac<Sha256>;

/// Constant-time slice comparison to prevent timing attacks.
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut result = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        result |= x ^ y;
    }
    result == 0
}

/// Verify Razorpay Webhook signature (`X-Razorpay-Signature` header).
///
/// Computes HMAC-SHA256 of the raw webhook body payload using your webhook secret,
/// and compares it in constant time with the incoming signature.
pub fn verify_webhook_signature(body: &str, signature: &str, secret: &str) -> RazorpayResult<()> {
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
        .map_err(|_| RazorpayError::InvalidInput("invalid webhook secret".into()))?;
    mac.update(body.as_bytes());
    let expected = hex::encode(mac.finalize().into_bytes());

    if constant_time_eq(expected.as_bytes(), signature.as_bytes()) {
        Ok(())
    } else {
        Err(RazorpayError::SignatureMismatch)
    }
}

/// Verify Razorpay standard Checkout payment signature (`razorpay_signature`).
///
/// Computes HMAC-SHA256 of `order_id|payment_id` using your API Key Secret.
pub fn verify_payment_signature(
    order_id: &str,
    payment_id: &str,
    signature: &str,
    secret: &str,
) -> RazorpayResult<()> {
    let payload = format!("{}|{}", order_id, payment_id);
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
        .map_err(|_| RazorpayError::InvalidInput("invalid key secret".into()))?;
    mac.update(payload.as_bytes());
    let expected = hex::encode(mac.finalize().into_bytes());

    if constant_time_eq(expected.as_bytes(), signature.as_bytes()) {
        Ok(())
    } else {
        Err(RazorpayError::SignatureMismatch)
    }
}

/// Verify Razorpay Subscription payment signature.
///
/// Computes HMAC-SHA256 of `payment_id|subscription_id` using your API Key Secret.
pub fn verify_subscription_payment_signature(
    subscription_id: &str,
    payment_id: &str,
    signature: &str,
    secret: &str,
) -> RazorpayResult<()> {
    let payload = format!("{}|{}", payment_id, subscription_id);
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
        .map_err(|_| RazorpayError::InvalidInput("invalid key secret".into()))?;
    mac.update(payload.as_bytes());
    let expected = hex::encode(mac.finalize().into_bytes());

    if constant_time_eq(expected.as_bytes(), signature.as_bytes()) {
        Ok(())
    } else {
        Err(RazorpayError::SignatureMismatch)
    }
}

// TODO("Add strongly-typed WebhookEvent enum matching event types like payment.captured, order.paid, subscription.charged, etc.")
// TODO("Add programmatic Webhook management resource (client.webhooks() CRUD endpoints hitting /v1/webhooks and /v2/accounts/{account_id}/webhooks)")
