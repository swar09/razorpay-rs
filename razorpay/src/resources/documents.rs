use async_trait::async_trait;
use reqwest::header::HeaderMap;
use std::sync::Arc;

use crate::{error::RazorpayResult, http::Http, models::Document, traits::Fetchable};

/// Resource handle for Razorpay Documents API endpoints (`/v1/documents`).
#[derive(Debug, Clone)]
pub struct Documents {
    pub(crate) http: Arc<Http>,
}

impl Documents {
    pub(crate) fn new(http: Arc<Http>) -> Self {
        Self { http }
    }
}

#[async_trait]
impl Fetchable for Documents {
    type Item = Document;

    /// Fetch a document by ID (`GET /v1/documents/{document_id}`).
    async fn fetch(
        &self,
        document_id: &str,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<Self::Item> {
        let path = format!("documents/{}", document_id);
        self.http
            .get::<Self::Item, ()>(&path, None, extra_headers)
            .await
    }
}
