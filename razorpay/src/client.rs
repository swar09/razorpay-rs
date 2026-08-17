use std::{sync::Arc, time::Duration};
use url::Url;

use crate::{
    config::{DEFAULT_BASE_URL, DEFAULT_TIMEOUT, RazorpayConfig},
    error::{RazorpayError, RazorpayResult},
    http::Http,
};

/// Main Razorpay SDK client.
///
/// Holds an internal `Arc<Http>` reference, making it cheap to clone across threads
/// and pass into async tasks or application state.
///
/// # Example
///
/// ```no_run
/// use razorpay::RazorpayClient;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = RazorpayClient::new("rzp_test_key", "test_secret")?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct RazorpayClient {
    pub(crate) http: Arc<Http>,
}

impl RazorpayClient {
    /// Create a new `RazorpayClient` with API credentials and default settings.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use razorpay::RazorpayClient;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = RazorpayClient::new("rzp_test_key", "test_secret")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(key_id: impl Into<String>, key_secret: impl Into<String>) -> RazorpayResult<Self> {
        RazorpayClientBuilder::new()
            .key_id(key_id)
            .key_secret(key_secret)
            .build()
    }

    /// Obtain a new builder instance to configure client options.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use razorpay::RazorpayClient;
    /// use std::time::Duration;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = RazorpayClient::builder()
    ///     .key_id("rzp_test_key")
    ///     .key_secret("test_secret")
    ///     .timeout(Duration::from_secs(45))
    ///     .build()?;
    /// # Ok(())
    /// # }
    /// ```
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

    /// Access Virtual Accounts / Smart Collect operations (`/v1/virtual_accounts`).
    pub fn virtual_accounts(&self) -> crate::resources::virtual_accounts::VirtualAccounts {
        crate::resources::virtual_accounts::VirtualAccounts::new(Arc::clone(&self.http))
    }

    /// Access QR Codes resource operations (`/v1/payments/qr_codes`).
    pub fn qr_codes(&self) -> crate::resources::qr_codes::QrCodes {
        crate::resources::qr_codes::QrCodes::new(Arc::clone(&self.http))
    }

    /// Access Items catalog resource operations (`/v1/items`).
    pub fn items(&self) -> crate::resources::items::Items {
        crate::resources::items::Items::new(Arc::clone(&self.http))
    }

    /// Access Disputes resource operations (`/v1/disputes`).
    pub fn disputes(&self) -> crate::resources::disputes::Disputes {
        crate::resources::disputes::Disputes::new(Arc::clone(&self.http))
    }

    /// Access Documents resource operations (`/v1/documents`).
    pub fn documents(&self) -> crate::resources::documents::Documents {
        crate::resources::documents::Documents::new(Arc::clone(&self.http))
    }

    /// Access Fund Accounts resource operations (`/v1/fund_accounts`).
    pub fn fund_accounts(&self) -> crate::resources::fund_accounts::FundAccounts {
        crate::resources::fund_accounts::FundAccounts::new(Arc::clone(&self.http))
    }

    /// Access Payouts resource operations (`/v1/payouts`).
    pub fn payouts(&self) -> crate::resources::payouts::Payouts {
        crate::resources::payouts::Payouts::new(Arc::clone(&self.http))
    }

    /// Access Cards resource operations (`/v1/cards`).
    pub fn cards(&self) -> crate::resources::cards::Cards {
        crate::resources::cards::Cards::new(Arc::clone(&self.http))
    }

    /// Access IINs (Issuer Identification Numbers) resource operations (`/v1/iins`).
    pub fn iins(&self) -> crate::resources::iins::Iins {
        crate::resources::iins::Iins::new(Arc::clone(&self.http))
    }

    /// Access Products configuration resource operations (`/v2/products`).
    pub fn products(&self) -> crate::resources::products::Products {
        crate::resources::products::Products::new(Arc::clone(&self.http))
    }

    /// Access Bills resource operations (`/v1/bills`).
    pub fn bills(&self) -> crate::resources::bills::Bills {
        crate::resources::bills::Bills::new(Arc::clone(&self.http))
    }

    /// Access Webhooks management resource operations (`/v1/webhooks` and `/v2/accounts/{account_id}/webhooks`).
    pub fn webhooks(&self) -> crate::resources::webhooks::Webhooks {
        crate::resources::webhooks::Webhooks::new(Arc::clone(&self.http))
    }

    /// Access Payment Methods API operations (`/v1/methods`).
    pub fn methods(&self) -> crate::resources::methods::Methods {
        crate::resources::methods::Methods::new(Arc::clone(&self.http))
    }

    /// Create a clone of this client configured to make API calls on behalf of a
    /// linked sub-merchant account via the `X-Razorpay-Account` header.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use razorpay::RazorpayClient;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = RazorpayClient::new("rzp_test_key", "test_secret")?;
    /// let sub_client = client.with_account("acc_1234567890ABCD")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_account(&self, account_id: impl AsRef<str>) -> RazorpayResult<Self> {
        let http = self.http.with_account_id(account_id.as_ref())?;
        Ok(Self {
            http: Arc::new(http),
        })
    }
}

/// Builder for configuring and creating a [`RazorpayClient`].
///
/// # Example
///
/// ```no_run
/// use razorpay::RazorpayClientBuilder;
/// use std::time::Duration;
/// use url::Url;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = RazorpayClientBuilder::new()
///     .key_id("rzp_test_key")
///     .key_secret("test_secret")
///     .base_url(Url::parse("https://api.razorpay.com/v1/")?)
///     .timeout(Duration::from_secs(30))
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Default, Clone)]
pub struct RazorpayClientBuilder {
    key_id: Option<String>,
    key_secret: Option<String>,
    base_url: Option<Url>,
    timeout: Option<Duration>,
    custom_client: Option<reqwest::Client>,
}

impl std::fmt::Debug for RazorpayClientBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RazorpayClientBuilder")
            .field("key_id", &self.key_id)
            .field(
                "key_secret",
                &self.key_secret.as_ref().map(|_| "[REDACTED]"),
            )
            .field("base_url", &self.base_url)
            .field("timeout", &self.timeout)
            .field("custom_client", &self.custom_client)
            .finish()
    }
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
        let key_id = self.key_id.ok_or(RazorpayError::Config("missing key_id"))?;
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
