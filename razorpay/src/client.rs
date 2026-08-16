use std::{sync::Arc, time::Duration};
use url::Url;

use crate::{
    config::{RazorpayConfig, DEFAULT_BASE_URL, DEFAULT_TIMEOUT},
    error::{RazorpayError, RazorpayResult},
    http::Http,
};

/// Main Razorpay SDK client.
///
/// Holds an internal `Arc<Http>` reference, making it cheap to clone across threads
/// and pass into async tasks or application state.
#[derive(Debug, Clone)]
pub struct RazorpayClient {
    pub(crate) http: Arc<Http>,
}

impl RazorpayClient {
    /// Create a new `RazorpayClient` with API credentials and default settings.
    pub fn new(key_id: impl Into<String>, key_secret: impl Into<String>) -> RazorpayResult<Self> {
        RazorpayClientBuilder::new()
            .key_id(key_id)
            .key_secret(key_secret)
            .build()
    }

    /// Obtain a new builder instance to configure client options.
    pub fn builder() -> RazorpayClientBuilder {
        RazorpayClientBuilder::new()
    }

    /// Access the underlying configuration.
    pub fn config(&self) -> &RazorpayConfig {
        &self.http.config
    }

    /// Access Orders resource operations (`/v1/orders`).
    pub fn orders(&self) -> crate::resources::orders::Orders {
        crate::resources::orders::Orders::new(Arc::clone(&self.http))
    }

    /// Access Payments resource operations (`/v1/payments`).
    pub fn payments(&self) -> crate::resources::payments::Payments {
        crate::resources::payments::Payments::new(Arc::clone(&self.http))
    }

    /// Access Refunds resource operations (`/v1/refunds`).
    pub fn refunds(&self) -> crate::resources::refunds::Refunds {
        crate::resources::refunds::Refunds::new(Arc::clone(&self.http))
    }

    /// Access Customers resource operations (`/v1/customers`).
    pub fn customers(&self) -> crate::resources::customers::Customers {
        crate::resources::customers::Customers::new(Arc::clone(&self.http))
    }

    /// Access Payment Links resource operations (`/v1/payment_links`).
    pub fn payment_links(&self) -> crate::resources::payment_links::PaymentLinks {
        crate::resources::payment_links::PaymentLinks::new(Arc::clone(&self.http))
    }

    /// Access Invoices resource operations (`/v1/invoices`).
    pub fn invoices(&self) -> crate::resources::invoices::Invoices {
        crate::resources::invoices::Invoices::new(Arc::clone(&self.http))
    }

    /// Access Tokens resource operations (`/v1/tokens`).
    pub fn tokens(&self) -> crate::resources::tokens::Tokens {
        crate::resources::tokens::Tokens::new(Arc::clone(&self.http))
    }

    /// Access Plans resource operations (`/v1/plans`).
    pub fn plans(&self) -> crate::resources::plans::Plans {
        crate::resources::plans::Plans::new(Arc::clone(&self.http))
    }

    /// Access Subscriptions resource operations (`/v1/subscriptions`).
    pub fn subscriptions(&self) -> crate::resources::subscriptions::Subscriptions {
        crate::resources::subscriptions::Subscriptions::new(Arc::clone(&self.http))
    }

    /// Access Addons resource operations (`/v1/addons`).
    pub fn addons(&self) -> crate::resources::addons::Addons {
        crate::resources::addons::Addons::new(Arc::clone(&self.http))
    }

    /// Access Settlements resource operations (`/v1/settlements`).
    pub fn settlements(&self) -> crate::resources::settlements::Settlements {
        crate::resources::settlements::Settlements::new(Arc::clone(&self.http))
    }

    /// Access Transfers (Route) resource operations (`/v1/transfers`).
    pub fn transfers(&self) -> crate::resources::transfers::Transfers {
        crate::resources::transfers::Transfers::new(Arc::clone(&self.http))
    }

    /// Access Linked Accounts (Route) resource operations (`/v2/accounts`).
    pub fn accounts(&self) -> crate::resources::accounts::Accounts {
        crate::resources::accounts::Accounts::new(Arc::clone(&self.http))
    }
}

/// Builder for constructing a configured [`RazorpayClient`].
#[derive(Debug, Default, Clone)]
pub struct RazorpayClientBuilder {
    key_id: Option<String>,
    key_secret: Option<String>,
    base_url: Option<Url>,
    timeout: Option<Duration>,
    custom_client: Option<reqwest::Client>,
}

impl RazorpayClientBuilder {
    /// Create a new empty builder instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the Razorpay Key ID (API Key).
    pub fn key_id(mut self, key_id: impl Into<String>) -> Self {
        self.key_id = Some(key_id.into());
        self
    }

    /// Set the Razorpay Key Secret.
    pub fn key_secret(mut self, key_secret: impl Into<String>) -> Self {
        self.key_secret = Some(key_secret.into());
        self
    }

    /// Set a custom base URL (e.g. for mock testing or staging environments).
    pub fn base_url(mut self, base_url: Url) -> Self {
        self.base_url = Some(base_url);
        self
    }

    /// Set a custom HTTP request timeout.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Provide a pre-configured `reqwest::Client`.
    pub fn custom_client(mut self, client: reqwest::Client) -> Self {
        self.custom_client = Some(client);
        self
    }

    /// Build and return the initialized [`RazorpayClient`].
    pub fn build(self) -> RazorpayResult<RazorpayClient> {
        let key_id = self
            .key_id
            .ok_or(RazorpayError::Config("missing key_id"))?;
        let key_secret = self
            .key_secret
            .ok_or(RazorpayError::Config("missing key_secret"))?;

        let base_url = match self.base_url {
            Some(url) => url,
            None => Url::parse(DEFAULT_BASE_URL)?,
        };

        let timeout = self.timeout.unwrap_or(DEFAULT_TIMEOUT);

        let config = RazorpayConfig {
            key_id,
            key_secret,
            base_url,
            timeout,
        };

        let http = match self.custom_client {
            Some(client) => Http::with_client(config, client),
            None => Http::new(config)?,
        };

        Ok(RazorpayClient {
            http: Arc::new(http),
        })
    }
}
