use async_trait::async_trait;
use reqwest::header::HeaderMap;
use std::sync::Arc;

use crate::{
    error::RazorpayResult,
    http::Http,
    models::{Iin, ListOptions, RazorpayList},
    traits::Fetchable,
};

/// Resource handle for Razorpay Issuer Identification Number (IIN) endpoints (`/v1/iins`).
#[derive(Debug, Clone)]
pub struct Iins {
    pub(crate) http: Arc<Http>,
}

impl Iins {
    pub(crate) fn new(http: Arc<Http>) -> Self {
        Self { http }
    }

    /// Fetch all IINs list (`GET /v1/iins/list`).
    pub async fn all(
        &self,
        query: Option<ListOptions>,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<RazorpayList<Iin>> {
        self.http.get("iins/list", query.as_ref(), extra_headers).await
    }
}

#[async_trait]
impl Fetchable for Iins {
    type Item = Iin;

    /// Fetch IIN card metadata (`GET /v1/iins/{token_iin}`).
    async fn fetch(
        &self,
        token_iin: &str,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<Self::Item> {
        let path = format!("iins/{}", token_iin);
        self.http
            .get::<Self::Item, ()>(&path, None, extra_headers)
            .await
    }
}
