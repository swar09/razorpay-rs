use reqwest::header::HeaderMap;
use std::sync::Arc;

use crate::{error::RazorpayResult, http::Http, models::PaymentMethods};

/// Resource handle for Razorpay Payment Methods API endpoints (`/v1/methods`).
///
/// Query enabled payment instruments, banks, card networks, and wallets supported on your Razorpay account.
#[derive(Debug, Clone)]
pub struct Methods {
    pub(crate) http: Arc<Http>,
}

impl Methods {
    pub(crate) fn new(http: Arc<Http>) -> Self {
        Self { http }
    }

    /// Fetch all enabled payment methods and configurations (`GET /v1/methods`).
    pub async fn all(&self, extra_headers: Option<HeaderMap>) -> RazorpayResult<PaymentMethods> {
        self.http
            .get::<PaymentMethods, ()>("methods", None, extra_headers)
            .await
    }
}
