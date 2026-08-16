use async_trait::async_trait;
use reqwest::header::HeaderMap;
use std::sync::Arc;

use crate::{
    error::RazorpayResult,
    http::Http,
    models::{CreatePlanRequest, ListOptions, Plan, RazorpayList},
    traits::{Creatable, Fetchable, Listable},
};

/// Resource handle for Razorpay Plans API endpoints (`/v1/plans`).
#[derive(Debug, Clone)]
pub struct Plans {
    pub(crate) http: Arc<Http>,
}

impl Plans {
    pub(crate) fn new(http: Arc<Http>) -> Self {
        Self { http }
    }
}

#[async_trait]
impl Creatable for Plans {
    type Request = CreatePlanRequest;
    type Response = Plan;

    /// Create a recurring billing plan (`POST /v1/plans`).
    async fn create(&self, data: Self::Request, extra_headers: Option<HeaderMap>) -> RazorpayResult<Self::Response> {
        self.http.post("plans", &data, extra_headers).await
    }
}

#[async_trait]
impl Fetchable for Plans {
    type Item = Plan;

    /// Fetch a plan by ID (`GET /v1/plans/{plan_id}`).
    async fn fetch(&self, plan_id: &str, extra_headers: Option<HeaderMap>) -> RazorpayResult<Self::Item> {
        let path = format!("plans/{}", plan_id);
        self.http.get::<Self::Item, ()>(&path, None, extra_headers).await
    }
}

#[async_trait]
impl Listable for Plans {
    type Item = Plan;

    /// Fetch a paginated collection of plans (`GET /v1/plans`).
    async fn all(
        &self,
        query: Option<ListOptions>,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<RazorpayList<Self::Item>> {
        self.http.get("plans", query.as_ref(), extra_headers).await
    }
}
