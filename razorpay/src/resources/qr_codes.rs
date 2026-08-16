use async_trait::async_trait;
use reqwest::header::HeaderMap;
use std::sync::Arc;

use crate::{
    error::RazorpayResult,
    http::Http,
    models::{CreateQrCodeRequest, ListOptions, Payment, QrCode, RazorpayList},
    traits::{Creatable, Fetchable, Listable},
};

/// Resource handle for Razorpay QR Codes API endpoints (`/v1/payments/qr_codes`).
#[derive(Debug, Clone)]
pub struct QrCodes {
    pub(crate) http: Arc<Http>,
}

impl QrCodes {
    pub(crate) fn new(http: Arc<Http>) -> Self {
        Self { http }
    }

    /// Close a QR Code (`POST /v1/payments/qr_codes/{qr_code_id}/close`).
    pub async fn close(
        &self,
        qr_code_id: &str,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<QrCode> {
        let path = format!("payments/qr_codes/{}/close", qr_code_id);
        self.http
            .post(&path, &serde_json::json!({}), extra_headers)
            .await
    }

    /// Fetch all payments received on a QR Code (`GET /v1/payments/qr_codes/{qr_code_id}/payments`).
    pub async fn payments(
        &self,
        qr_code_id: &str,
        query: Option<ListOptions>,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<RazorpayList<Payment>> {
        let path = format!("payments/qr_codes/{}/payments", qr_code_id);
        self.http.get(&path, query.as_ref(), extra_headers).await
    }
}

#[async_trait]
impl Creatable for QrCodes {
    type Request = CreateQrCodeRequest;
    type Response = QrCode;

    /// Create a QR code (`POST /v1/payments/qr_codes`).
    async fn create(
        &self,
        data: Self::Request,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<Self::Response> {
        self.http
            .post("payments/qr_codes", &data, extra_headers)
            .await
    }
}

#[async_trait]
impl Fetchable for QrCodes {
    type Item = QrCode;

    /// Fetch a QR code by ID (`GET /v1/payments/qr_codes/{qr_code_id}`).
    async fn fetch(
        &self,
        qr_code_id: &str,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<Self::Item> {
        let path = format!("payments/qr_codes/{}", qr_code_id);
        self.http
            .get::<Self::Item, ()>(&path, None, extra_headers)
            .await
    }
}

#[async_trait]
impl Listable for QrCodes {
    type Item = QrCode;

    /// Fetch all QR codes (`GET /v1/payments/qr_codes`).
    async fn all(
        &self,
        query: Option<ListOptions>,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<RazorpayList<Self::Item>> {
        self.http
            .get("payments/qr_codes", query.as_ref(), extra_headers)
            .await
    }
}
