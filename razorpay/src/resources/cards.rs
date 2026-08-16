use async_trait::async_trait;
use reqwest::header::HeaderMap;
use std::sync::Arc;

use crate::{
    error::RazorpayResult,
    http::Http,
    models::Card,
    traits::Fetchable,
};

/// Resource handle for Razorpay Cards API endpoints (`/v1/cards`).
#[derive(Debug, Clone)]
pub struct Cards {
    pub(crate) http: Arc<Http>,
}

impl Cards {
    pub(crate) fn new(http: Arc<Http>) -> Self {
        Self { http }
    }

    /// Request card reference / fingerprints (`POST /v1/cards/fingerprints`).
    pub async fn request_card_reference<T: serde::Serialize + Sync>(
        &self,
        data: &T,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<serde_json::Value> {
        self.http.post("cards/fingerprints", data, extra_headers).await
    }
}

#[async_trait]
impl Fetchable for Cards {
    type Item = Card;

    /// Fetch card details by ID (`GET /v1/cards/{card_id}`).
    async fn fetch(
        &self,
        card_id: &str,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<Self::Item> {
        let path = format!("cards/{}", card_id);
        self.http
            .get::<Self::Item, ()>(&path, None, extra_headers)
            .await
    }
}
