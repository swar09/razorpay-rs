use async_trait::async_trait;
use reqwest::header::HeaderMap;
use std::sync::Arc;

use crate::{
    error::RazorpayResult,
    http::Http,
    models::{CreatePayoutRequest, ListOptions, Payout, RazorpayList},
    traits::{Creatable, Fetchable, Listable},
};

/// Resource handle for RazorpayX Payouts API endpoints (`/v1/payouts`).
#[derive(Debug, Clone)]
pub struct Payouts {
    pub(crate) http: Arc<Http>,
}

impl Payouts {
    pub(crate) fn new(http: Arc<Http>) -> Self {
        Self { http }
    }

    /// Cancel a queued payout (`POST /v1/payouts/{payout_id}/cancel`).
    pub async fn cancel(
        &self,
        payout_id: &str,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<Payout> {
        let path = format!("payouts/{}/cancel", payout_id);
        self.http
            .post(&path, &serde_json::json!({}), extra_headers)
            .await
    }
}

#[async_trait]
impl Creatable for Payouts {
    type Request = CreatePayoutRequest;
    type Response = Payout;

    /// Create a payout (`POST /v1/payouts`).
    async fn create(
        &self,
        data: Self::Request,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<Self::Response> {
        self.http.post("payouts", &data, extra_headers).await
    }
}

#[async_trait]
impl Fetchable for Payouts {
    type Item = Payout;

    /// Fetch a payout by ID (`GET /v1/payouts/{payout_id}`).
    async fn fetch(
        &self,
        payout_id: &str,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<Self::Item> {
        let path = format!("payouts/{}", payout_id);
        self.http
            .get::<Self::Item, ()>(&path, None, extra_headers)
            .await
    }
}

#[async_trait]
impl Listable for Payouts {
    type Item = Payout;

    /// Fetch all payouts (`GET /v1/payouts`).
    async fn all(
        &self,
        query: Option<ListOptions>,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<RazorpayList<Self::Item>> {
        self.http
            .get("payouts", query.as_ref(), extra_headers)
            .await
    }
}
