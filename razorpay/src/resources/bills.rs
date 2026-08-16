use async_trait::async_trait;
use reqwest::header::HeaderMap;
use std::sync::Arc;

use crate::{
    error::RazorpayResult,
    http::Http,
    models::{Bill, DeleteResponse},
    traits::{Creatable, Deletable, Fetchable, Updatable},
};

/// Resource handle for Razorpay Bills API endpoints (`/v1/bills`).
///
/// https://razorpay.com/docs/api/payments/bills
#[derive(Debug, Clone)]
pub struct Bills {
    pub(crate) http: Arc<Http>,
}

impl Bills {
    pub(crate) fn new(http: Arc<Http>) -> Self {
        Self { http }
    }
}

#[async_trait]
impl Creatable for Bills {
    type Request = serde_json::Value;
    type Response = Bill;

    /// Create a new bill (`POST /v1/bills`).
    async fn create(
        &self,
        data: Self::Request,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<Self::Response> {
        self.http.post("bills", &data, extra_headers).await
    }
}

#[async_trait]
impl Fetchable for Bills {
    type Item = Bill;

    /// Fetch a bill by its ID (`GET /v1/bills/{bill_id}`).
    async fn fetch(
        &self,
        bill_id: &str,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<Self::Item> {
        let path = format!("bills/{}", bill_id);
        self.http
            .get::<Self::Item, ()>(&path, None, extra_headers)
            .await
    }
}

#[async_trait]
impl Updatable for Bills {
    type Request = serde_json::Value;
    type Response = Bill;

    /// Update a bill (`PATCH /v1/bills/{bill_id}`).
    async fn update(
        &self,
        bill_id: &str,
        data: Self::Request,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<Self::Response> {
        let path = format!("bills/{}", bill_id);
        self.http.patch(&path, &data, extra_headers).await
    }
}

#[async_trait]
impl Deletable for Bills {
    type Response = DeleteResponse;

    /// Delete a bill (`DELETE /v1/bills/{bill_id}`).
    async fn delete(
        &self,
        bill_id: &str,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<Self::Response> {
        let path = format!("bills/{}", bill_id);
        self.http.delete(&path, extra_headers).await
    }
}
