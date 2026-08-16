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

    /// Upload a document file via multipart form (`POST /v1/documents`).
    ///
    /// Accepts a file path and a document purpose string (e.g., `"dispute_evidence"`, `"kyc"`).
    /// Returns the uploaded [`Document`] record containing the assigned ID (`doc_xxx`).
    pub async fn create(
        &self,
        file_path: impl AsRef<std::path::Path>,
        purpose: &str,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<Document> {
        let path = file_path.as_ref();
        let filename = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("document")
            .to_string();
        let bytes = std::fs::read(path).map_err(crate::error::RazorpayError::Io)?;
        self.create_from_bytes(bytes, &filename, purpose, extra_headers)
            .await
    }

    /// Upload a document from in-memory raw bytes (`POST /v1/documents`).
    pub async fn create_from_bytes(
        &self,
        file_bytes: Vec<u8>,
        filename: &str,
        purpose: &str,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<Document> {
        let part = reqwest::multipart::Part::bytes(file_bytes)
            .file_name(filename.to_string());

        let form = reqwest::multipart::Form::new()
            .text("purpose", purpose.to_string())
            .part("file", part);

        self.http.multipart("documents", form, extra_headers).await
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
