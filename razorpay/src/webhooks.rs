use hmac::{Hmac, Mac};
use sha2::Sha256;

use crate::{
    error::{RazorpayError, RazorpayResult},
    models::WebhookPayload,
};

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

/// Parse and deserialize a raw incoming Razorpay Webhook payload.
///
/// Ensures valid JSON structure and converts to strongly-typed [`WebhookPayload`].
pub fn parse_webhook_event(raw_body: &str) -> RazorpayResult<WebhookPayload> {
    serde_json::from_str::<WebhookPayload>(raw_body).map_err(RazorpayError::Serde)
}

/// Verify signature and parse an incoming webhook payload in a single step.
pub fn verify_and_parse_webhook(
    body: &str,
    signature: &str,
    secret: &str,
) -> RazorpayResult<WebhookPayload> {
    verify_webhook_signature(body, signature, secret)?;
    parse_webhook_event(body)
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
/// Parameters are specified in payload order (`payment_id`, then `subscription_id`).
pub fn verify_subscription_payment_signature(
    payment_id: &str,
    subscription_id: &str,
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

/// Helper struct for type-safe Checkout payment signature verification.
#[derive(Clone)]
pub struct PaymentSignatureVerification<'a> {
    pub order_id: &'a str,
    pub payment_id: &'a str,
    pub signature: &'a str,
    pub secret: &'a str,
}

impl std::fmt::Debug for PaymentSignatureVerification<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PaymentSignatureVerification")
            .field("order_id", &self.order_id)
            .field("payment_id", &self.payment_id)
            .field("signature", &self.signature)
            .field("secret", &"[REDACTED]")
            .finish()
    }
}

impl<'a> PaymentSignatureVerification<'a> {
    pub fn verify(&self) -> RazorpayResult<()> {
        verify_payment_signature(self.order_id, self.payment_id, self.signature, self.secret)
    }
}

/// Helper struct for type-safe Subscription payment signature verification.
#[derive(Clone)]
pub struct SubscriptionPaymentSignatureVerification<'a> {
    pub payment_id: &'a str,
    pub subscription_id: &'a str,
    pub signature: &'a str,
    pub secret: &'a str,
}

impl std::fmt::Debug for SubscriptionPaymentSignatureVerification<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SubscriptionPaymentSignatureVerification")
            .field("payment_id", &self.payment_id)
            .field("subscription_id", &self.subscription_id)
            .field("signature", &self.signature)
            .field("secret", &"[REDACTED]")
            .finish()
    }
}

impl<'a> SubscriptionPaymentSignatureVerification<'a> {
    pub fn verify(&self) -> RazorpayResult<()> {
        verify_subscription_payment_signature(
            self.payment_id,
            self.subscription_id,
            self.signature,
            self.secret,
        )
    }
}

/// Verify Razorpay Payment Link payment signature.
///
/// Computes HMAC-SHA256 of `payment_link_id|payment_link_reference_id|payment_link_status|payment_id`
/// using your API Key Secret.
pub fn verify_payment_link_signature(
    payment_link_id: &str,
    payment_link_reference_id: &str,
    payment_link_status: &str,
    payment_id: &str,
    signature: &str,
    secret: &str,
) -> RazorpayResult<()> {
    let payload = format!(
        "{}|{}|{}|{}",
        payment_link_id, payment_link_reference_id, payment_link_status, payment_id
    );
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

/// Helper struct for type-safe Payment Link signature verification.
#[derive(Clone)]
pub struct PaymentLinkSignatureVerification<'a> {
    pub payment_link_id: &'a str,
    pub payment_link_reference_id: &'a str,
    pub payment_link_status: &'a str,
    pub payment_id: &'a str,
    pub signature: &'a str,
    pub secret: &'a str,
}

impl std::fmt::Debug for PaymentLinkSignatureVerification<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PaymentLinkSignatureVerification")
            .field("payment_link_id", &self.payment_link_id)
            .field("payment_link_reference_id", &self.payment_link_reference_id)
            .field("payment_link_status", &self.payment_link_status)
            .field("payment_id", &self.payment_id)
            .field("signature", &self.signature)
            .field("secret", &"[REDACTED]")
            .finish()
    }
}

impl<'a> PaymentLinkSignatureVerification<'a> {
    pub fn verify(&self) -> RazorpayResult<()> {
        verify_payment_link_signature(
            self.payment_link_id,
            self.payment_link_reference_id,
            self.payment_link_status,
            self.payment_id,
            self.signature,
            self.secret,
        )
    }
}
