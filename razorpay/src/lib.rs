//! # razorpay-rs
//!
//! An async, type-safe, idiomatic Rust SDK for the [Razorpay API](https://razorpay.com/docs/api/).
//!
//! ## Overview
//!
//! `razorpay-rs` provides access to Razorpay's payments platform including:
//! - **Orders & Payments**: Create orders, capture payments, handle OTPs, and issue refunds.
//! - **Payment Links & Invoices**: Generate hosted payment links and invoice line items.
//! - **Subscriptions & Plans**: Set up recurring billing cycles, addons, and e-mandates.
//! - **Smart Collect & Virtual Accounts**: Create virtual bank accounts and UPI VPAs.
//! - **Route & Transfers**: Split payments and manage linked marketplace accounts.
//! - **Disputes & Settlements**: Track chargebacks and on-demand instant settlements.
//! - **Bills**: Manage retail and point-of-sale (POS) digital receipts.
//! - **Webhooks & Security**: Constant-time signature verification for webhooks and checkouts.
//!
//! ## Creating a Client
//!
//! ### Basic Initialization
//! ```no_run
//! use razorpay::RazorpayClient;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = RazorpayClient::new("rzp_test_key", "test_secret")?;
//! # Ok(())
//! # }
//! ```
//!
//! ### Custom Configuration with Builder
//! ```no_run
//! use razorpay::RazorpayClientBuilder;
//! use std::time::Duration;
//! use url::Url;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = RazorpayClientBuilder::new()
//!     .key_id("rzp_test_key")
//!     .key_secret("test_secret")
//!     .timeout(Duration::from_secs(30))
//!     .build()?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Usage Examples
//!
//! ### Creating an Order
//! ```no_run
//! use razorpay::{
//!     Creatable, RazorpayClient,
//!     models::CreateOrderRequest,
//! };
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = RazorpayClient::new("rzp_test_key", "test_secret")?;
//!
//! let req = CreateOrderRequest {
//!     amount: 50000, // 500.00 INR in paise
//!     currency: "INR".to_string(),
//!     receipt: Some("rcpt_101".to_string()),
//!     partial_payment: Some(false),
//!     first_payment_min_amount: None,
//!     transfers: None,
//!     notes: None,
//! };
//!
//! let order = client.orders().create(req, None).await?;
//! println!("Created order ID: {}", order.id);
//! # Ok(())
//! # }
//! ```
//!
//! ### Creating a Hosted Payment Link
//! ```no_run
//! use razorpay::{
//!     Creatable, RazorpayClient,
//!     models::CreatePaymentLinkRequest,
//! };
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = RazorpayClient::new("rzp_test_key", "test_secret")?;
//!
//! let req = CreatePaymentLinkRequest {
//!     amount: 25000, // 250.00 INR in paise
//!     currency: Some("INR".to_string()),
//!     accept_partial: Some(false),
//!     first_min_partial_amount: None,
//!     expire_by: None,
//!     reference_id: Some("tx_9981".to_string()),
//!     description: Some("Product Purchase Link".to_string()),
//!     customer: None,
//!     notify: None,
//!     reminder_enable: None,
//!     notes: None,
//!     callback_url: None,
//!     callback_method: None,
//! };
//!
//! let link = client.payment_links().create(req, None).await?;
//! println!("Payment Link URL: {}", link.short_url);
//! # Ok(())
//! # }
//! ```
//!
//! ### Verifying Checkout Payment Signature
//! ```no_run
//! use razorpay::webhooks::verify_payment_signature;
//!
//! let order_id = "order_EKwxwAgItmmXdp";
//! let payment_id = "pay_29AeabbJyL3mAO";
//! let signature = "9ef4dffbfd84f1318f6739a3ce19f9d85851857ae648f114332d840193e13ff1";
//! let key_secret = "test_secret";
//!
//! match verify_payment_signature(order_id, payment_id, signature, key_secret) {
//!     Ok(()) => println!("Payment verified successfully!"),
//!     Err(err) => eprintln!("Invalid signature: {:?}", err),
//! }
//! ```
//!
//! ### Verifying Webhook Signature
//! ```no_run
//! use razorpay::webhooks::verify_webhook_signature;
//!
//! let raw_body = r#"{"entity":"event","event":"payment.captured"}"#;
//! let signature_header = "25134763133642c26279f041c2c31e4e138a4d46f5de019e0cc0ab961a8a25c1";
//! let webhook_secret = "my_webhook_secret";
//!
//! match verify_webhook_signature(raw_body, signature_header, webhook_secret) {
//!     Ok(()) => println!("Webhook verified successfully!"),
//!     Err(err) => eprintln!("Webhook signature mismatch: {:?}", err),
//! }
//! ```

pub mod client;
pub mod config;
pub mod error;
pub(crate) mod http;
pub mod models;
pub mod resources;
pub mod traits;
pub mod webhooks;

// Convenient re-exports at root
pub use client::{RazorpayClient, RazorpayClientBuilder};
pub use config::RazorpayConfig;
pub use error::{RazorpayError, RazorpayResult};
pub use resources::{
    Accounts, Addons, Bills, Cards, CustomerTokens, Customers, Disputes, Documents, FundAccounts,
    Iins, Invoices, Items, Orders, PaymentLinks, Payments, Payouts, Plans, Products, QrCodes,
    Refunds, Settlements, Stakeholders, Subscriptions, Tokens, Transfers, VirtualAccounts,
    Webhooks,
};
pub use traits::{Creatable, Deletable, Fetchable, Listable, Updatable};
