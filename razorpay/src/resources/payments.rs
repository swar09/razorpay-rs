use async_trait::async_trait;
use reqwest::header::HeaderMap;
use std::sync::Arc;

use crate::{
    error::RazorpayResult,
    http::Http,
    models::{
        CapturePaymentRequest, CardDetails, CreateRefundRequest, ListOptions, Payment,
        PaymentDowntime, RazorpayList, Refund, Transfer, UpdatePaymentRequest,
    },
    traits::{Fetchable, Listable, Updatable},
};

/// Resource handle for Razorpay Payments API endpoints (`/v1/payments`).
#[derive(Debug, Clone)]
pub struct Payments {
    pub(crate) http: Arc<Http>,
}

impl Payments {
    pub(crate) fn new(http: Arc<Http>) -> Self {
        Self { http }
    }

    /// Capture an authorized payment (`POST /v1/payments/{payment_id}/capture`).
    pub async fn capture(
        &self,
        payment_id: &str,
        data: CapturePaymentRequest,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<Payment> {
        let path = format!("payments/{}/capture", payment_id);
        self.http.post(&path, &data, extra_headers).await
    }

    /// Issue a refund for a payment (`POST /v1/payments/{payment_id}/refund`).
    pub async fn refund(
        &self,
        payment_id: &str,
        data: CreateRefundRequest,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<Refund> {
        let path = format!("payments/{}/refund", payment_id);
        self.http.post(&path, &data, extra_headers).await
    }

    /// Fetch all refunds for a specific payment (`GET /v1/payments/{payment_id}/refunds`).
    pub async fn refunds(
        &self,
        payment_id: &str,
        query: Option<ListOptions>,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<RazorpayList<Refund>> {
        let path = format!("payments/{}/refunds", payment_id);
        self.http.get(&path, query.as_ref(), extra_headers).await
    }

    /// Fetch all transfers created for a specific payment (`GET /v1/payments/{payment_id}/transfers`).
    pub async fn transfers(
        &self,
        payment_id: &str,
        query: Option<ListOptions>,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<RazorpayList<Transfer>> {
        let path = format!("payments/{}/transfers", payment_id);
        self.http.get(&path, query.as_ref(), extra_headers).await
    }

    /// Fetch card details of a payment (`GET /v1/payments/{payment_id}/card`).
    pub async fn card_details(
        &self,
        payment_id: &str,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<CardDetails> {
        let path = format!("payments/{}/card", payment_id);
        self.http
            .get::<CardDetails, ()>(&path, None, extra_headers)
            .await
    }

    /// Fetch BankTransfer details associated with a payment (`GET /v1/payments/{payment_id}/bank_transfer`).
    pub async fn bank_transfer(
        &self,
        payment_id: &str,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<serde_json::Value> {
        let path = format!("payments/{}/bank_transfer", payment_id);
        self.http
            .get::<serde_json::Value, ()>(&path, None, extra_headers)
            .await
    }

    /// Fetch all payment downtime records (`GET /v1/payments/downtimes`).
    pub async fn fetch_downtime(
        &self,
        query: Option<ListOptions>,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<RazorpayList<PaymentDowntime>> {
        self.http
            .get("payments/downtimes", query.as_ref(), extra_headers)
            .await
    }

    /// Fetch payment downtime record by its ID (`GET /v1/payments/downtimes/{downtime_id}`).
    pub async fn fetch_downtime_by_id(
        &self,
        downtime_id: &str,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<PaymentDowntime> {
        let path = format!("payments/downtimes/{}", downtime_id);
        self.http
            .get::<PaymentDowntime, ()>(&path, None, extra_headers)
            .await
    }
}

#[async_trait]
impl Fetchable for Payments {
    type Item = Payment;

    /// Fetch a payment by its ID (`GET /v1/payments/{payment_id}`).
    async fn fetch(
        &self,
        payment_id: &str,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<Self::Item> {
        let path = format!("payments/{}", payment_id);
        self.http
            .get::<Self::Item, ()>(&path, None, extra_headers)
            .await
    }
}

#[async_trait]
impl Listable for Payments {
    type Item = Payment;

    /// Fetch a paginated list of payments (`GET /v1/payments`).
    async fn all(
        &self,
        query: Option<ListOptions>,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<RazorpayList<Self::Item>> {
        self.http
            .get("payments", query.as_ref(), extra_headers)
            .await
    }
}

#[async_trait]
impl Updatable for Payments {
    type Request = UpdatePaymentRequest;
    type Response = Payment;

    /// Update payment notes (`PATCH /v1/payments/{payment_id}`).
    async fn update(
        &self,
        payment_id: &str,
        data: Self::Request,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<Self::Response> {
        let path = format!("payments/{}", payment_id);
        self.http.patch(&path, &data, extra_headers).await
    }
}
