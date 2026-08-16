use async_trait::async_trait;
use reqwest::header::HeaderMap;
use std::sync::Arc;

use crate::{
    error::RazorpayResult,
    http::Http,
    models::{CreateRefundRequest, ListOptions, Refund, RazorpayList, UpdateRefundRequest},
    traits::{Creatable, Fetchable, Listable, Updatable},
};

/// Resource handle for Razorpay Refunds API endpoints (`/v1/refunds`).
#[derive(Debug, Clone)]
pub struct Refunds {
    pub(crate) http: Arc<Http>,
}

impl Refunds {
    pub(crate) fn new(http: Arc<Http>) -> Self {
        Self { http }
    }
}

#[async_trait]
impl Creatable for Refunds {
    type Request = CreateRefundRequest;
    type Response = Refund;

    /// Create a standalone refund (`POST /v1/refunds`).
    async fn create(&self, data: Self::Request, extra_headers: Option<HeaderMap>) -> RazorpayResult<Self::Response> {
        self.http.post("refunds", &data, extra_headers).await
    }
}

#[async_trait]
impl Fetchable for Refunds {
    type Item = Refund;

    /// Fetch a refund by its ID (`GET /v1/refunds/{refund_id}`).
    async fn fetch(&self, refund_id: &str, extra_headers: Option<HeaderMap>) -> RazorpayResult<Self::Item> {
        let path = format!("refunds/{}", refund_id);
        self.http.get::<Self::Item, ()>(&path, None, extra_headers).await
    }
}

#[async_trait]
impl Listable for Refunds {
    type Item = Refund;

    /// Fetch a paginated list of all refunds (`GET /v1/refunds`).
    async fn all(
        &self,
        query: Option<ListOptions>,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<RazorpayList<Self::Item>> {
        self.http.get("refunds", query.as_ref(), extra_headers).await
    }
}

#[async_trait]
impl Updatable for Refunds {
    type Request = UpdateRefundRequest;
    type Response = Refund;

    /// Update an existing refund (notes) (`PATCH /v1/refunds/{refund_id}`).
    async fn update(
        &self,
        refund_id: &str,
        data: Self::Request,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<Self::Response> {
        let path = format!("refunds/{}", refund_id);
        self.http.patch(&path, &data, extra_headers).await
    }
}
