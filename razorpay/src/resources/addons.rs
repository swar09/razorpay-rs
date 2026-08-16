use async_trait::async_trait;
use reqwest::header::HeaderMap;
use std::sync::Arc;

use crate::{
    error::RazorpayResult,
    http::Http,
    models::{Addon, DeleteResponse, ListOptions, RazorpayList},
    traits::{Deletable, Fetchable, Listable},
};

/// Resource handle for Razorpay Addons API endpoints (`/v1/addons`).
#[derive(Debug, Clone)]
pub struct Addons {
    pub(crate) http: Arc<Http>,
}

impl Addons {
    pub(crate) fn new(http: Arc<Http>) -> Self {
        Self { http }
    }
}

#[async_trait]
impl Fetchable for Addons {
    type Item = Addon;

    /// Fetch an addon by ID (`GET /v1/addons/{addon_id}`).
    async fn fetch(&self, addon_id: &str, extra_headers: Option<HeaderMap>) -> RazorpayResult<Self::Item> {
        let path = format!("addons/{}", addon_id);
        self.http.get::<Self::Item, ()>(&path, None, extra_headers).await
    }
}

#[async_trait]
impl Listable for Addons {
    type Item = Addon;

    /// Fetch a paginated collection of addons (`GET /v1/addons`).
    async fn all(
        &self,
        query: Option<ListOptions>,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<RazorpayList<Self::Item>> {
        self.http.get("addons", query.as_ref(), extra_headers).await
    }
}

#[async_trait]
impl Deletable for Addons {
    type Response = DeleteResponse;

    /// Delete an addon (`DELETE /v1/addons/{addon_id}`).
    async fn delete(&self, addon_id: &str, extra_headers: Option<HeaderMap>) -> RazorpayResult<Self::Response> {
        let path = format!("addons/{}", addon_id);
        self.http.delete(&path, extra_headers).await
    }
}
