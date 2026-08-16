# razorpay-rs

An async, type-safe, idiomatic Rust SDK for the [Razorpay API](https://razorpay.com/docs/api/).

[![CI](https://github.com/swar09/razorpay-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/swar09/razorpay-rs/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
<!-- [![Crates.io](https://img.shields.io/badge/crates.io-razorpay--rs-orange.svg)](https://crates.io/crates/razorpay-rs) -->

---

## Installation

Add `razorpay-rs` to your `Cargo.toml`:

```toml
[dependencies]
razorpay = { package = "razorpay-rs", version = "0.1.0-alpha.1" }
tokio = { version = "1", features = ["full"] }
```

Or using `cargo add`:

```bash
cargo add razorpay-rs@0.1.0-alpha.1
```

---

## Quickstart

```rust,no_run
use razorpay::{
    Creatable, RazorpayClient,
    models::CreateOrderRequest,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the client
    let client = RazorpayClient::new("rzp_test_key", "test_secret")?;

    // Create an order
    let req = CreateOrderRequest {
        amount: 50000, // 500.00 INR (in paise)
        currency: "INR".to_string(),
        receipt: Some("rcpt_001".to_string()),
        partial_payment: Some(false),
        first_payment_min_amount: None,
        transfers: None,
        notes: None,
    };

    let order = client.orders().create(req, None).await?;
    println!("Order created with ID: {}", order.id);

    Ok(())
}
```

---

## Verifying Signatures

### Checkout Payment Signature
```rust,no_run
use razorpay::webhooks::verify_payment_signature;

let order_id = "order_EKwxwAgItmmXdp";
let payment_id = "pay_29AeabbJyL3mAO";
let signature = "9ef4dffbfd84f1318f6739a3ce19f9d85851857ae648f114332d840193e13ff1";
let secret = "test_secret";

verify_payment_signature(order_id, payment_id, signature, secret)?;
```

### Webhook Signature
```rust,no_run
use razorpay::webhooks::verify_webhook_signature;

let payload = r#"{"entity":"event","event":"payment.captured"}"#;
let signature_header = "25134763133642c26279f041c2c31e4e138a4d46f5de019e0cc0ab961a8a25c1";
let webhook_secret = "your_webhook_secret";

verify_webhook_signature(payload, signature_header, webhook_secret)?;
```

---

## Testing

```bash
# Run unit & offline mock tests (100% offline)
cargo test-unit

# Run documentation tests
cargo test --doc

# Run live API integration tests against api.razorpay.com (requires .env keys)
cargo test-live
```

---

## Disclaimer

This is an unofficial, community-maintained Rust SDK and is not affiliated with, endorsed by, or sponsored by Razorpay Software Pvt. Ltd. "Razorpay" is a registered trademark of Razorpay Software Pvt. Ltd. and is used in this project solely to describe API compatibility. This software is provided "as is" without warranty of any kind, and users are solely responsible for compliance with Razorpay's official API Terms of Service. It is strongly recommended to thoroughly test all payment flows in Razorpay's sandbox/test mode before deploying to production environments.

---

## License

Licensed under the [MIT License](LICENSE).
