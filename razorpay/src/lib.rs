//! # razorpay-rs
//!
//! An async, type-safe, idiomatic Rust SDK for the [Razorpay API](https://razorpay.com/docs/api/).
//!
//! ## Quick Start
//!
//! ```no_run
//! use razorpay::RazorpayClient;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = RazorpayClient::new("rzp_test_key", "test_secret")?;
//! # Ok(())
//! # }
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
};
pub use traits::{Creatable, Deletable, Fetchable, Listable, Updatable};
