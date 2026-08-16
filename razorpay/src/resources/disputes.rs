use async_trait::async_trait;
use reqwest::header::HeaderMap;
use std::sync::Arc;

use crate::{
    error::RazorpayResult,
    http::Http,
    models::{ContestDisputeRequest, Dispute, ListOptions, RazorpayList},
    traits::{Fetchable, Listable},
};

/// Resource handle for Razorpay Disputes API endpoints (`/v1/disputes`).
#[derive(Debug, Clone)]
pub struct Disputes {
    pub(crate) http: Arc<Http>,
}

impl Disputes {
    pub(crate) fn new(http: Arc<Http>) -> Self {
        Self { http }
    }

    /// Accept a dispute (`POST /v1/disputes/{dispute_id}/accept`).
    pub async fn accept(
        &self,
        dispute_id: &str,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<Dispute> {
        let path = format!("disputes/{}/accept", dispute_id);
        self.http
            .post(&path, &serde_json::json!({}), extra_headers)
            .await
    }

    /// Contest a dispute with evidence (`PATCH /v1/disputes/{dispute_id}/contest`).
    pub async fn contest(
        &self,
        dispute_id: &str,
        data: ContestDisputeRequest,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<Dispute> {
        let path = format!("disputes/{}/contest", dispute_id);
        self.http.patch(&path, &data, extra_headers).await
    }
}

#[async_trait]
impl Fetchable for Disputes {
    type Item = Dispute;

    /// Fetch a dispute by ID (`GET /v1/disputes/{dispute_id}`).
    async fn fetch(
        &self,
        dispute_id: &str,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<Self::Item> {
        let path = format!("disputes/{}", dispute_id);
        self.http
            .get::<Self::Item, ()>(&path, None, extra_headers)
            .await
    }
}

#[async_trait]
impl Listable for Disputes {
    type Item = Dispute;

    /// Fetch all disputes (`GET /v1/disputes`).
    async fn all(
        &self,
        query: Option<ListOptions>,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<RazorpayList<Self::Item>> {
        self.http
            .get("disputes", query.as_ref(), extra_headers)
            .await
    }
}
