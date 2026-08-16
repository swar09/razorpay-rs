use async_trait::async_trait;
use reqwest::header::HeaderMap;
use std::sync::Arc;

use crate::{
    error::RazorpayResult,
    http::Http,
    models::{
        CreatePaymentLinkRequest, EditPaymentLinkRequest, ListOptions, NotifyMedium, PaymentLink,
        RazorpayList,
    },
    traits::{Creatable, Fetchable, Listable, Updatable},
};

/// Resource handle for Razorpay Payment Links API endpoints (`/v1/payment_links`).
#[derive(Debug, Clone)]
pub struct PaymentLinks {
    pub(crate) http: Arc<Http>,
}

impl PaymentLinks {
    pub(crate) fn new(http: Arc<Http>) -> Self {
        Self { http }
    }

    /// Cancel an active payment link (`POST /v1/payment_links/{payment_link_id}/cancel`).
    pub async fn cancel(
        &self,
        payment_link_id: &str,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<PaymentLink> {
        let path = format!("payment_links/{}/cancel", payment_link_id);
        self.http.post(&path, &serde_json::json!({}), extra_headers).await
    }

    /// Resend notification for a payment link via SMS or Email (`POST /v1/payment_links/{payment_link_id}/notify_by/{medium}`).
    pub async fn notify_by(
        &self,
        payment_link_id: &str,
        medium: NotifyMedium,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<serde_json::Value> {
        let medium_str = match medium {
            NotifyMedium::Sms => "sms",
            NotifyMedium::Email => "email",
        };
        let path = format!("payment_links/{}/notify_by/{}", payment_link_id, medium_str);
        self.http.post(&path, &serde_json::json!({}), extra_headers).await
    }
}

#[async_trait]
impl Creatable for PaymentLinks {
    type Request = CreatePaymentLinkRequest;
    type Response = PaymentLink;

    /// Create a standard or UPI payment link (`POST /v1/payment_links`).
    async fn create(&self, data: Self::Request, extra_headers: Option<HeaderMap>) -> RazorpayResult<Self::Response> {
        self.http.post("payment_links", &data, extra_headers).await
    }
}

#[async_trait]
impl Fetchable for PaymentLinks {
    type Item = PaymentLink;

    /// Fetch a payment link by ID (`GET /v1/payment_links/{payment_link_id}`).
    async fn fetch(&self, payment_link_id: &str, extra_headers: Option<HeaderMap>) -> RazorpayResult<Self::Item> {
        let path = format!("payment_links/{}", payment_link_id);
        self.http.get::<Self::Item, ()>(&path, None, extra_headers).await
    }
}

#[async_trait]
impl Listable for PaymentLinks {
    type Item = PaymentLink;

    /// Fetch a paginated collection of payment links (`GET /v1/payment_links`).
    async fn all(
        &self,
        query: Option<ListOptions>,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<RazorpayList<Self::Item>> {
        self.http.get("payment_links", query.as_ref(), extra_headers).await
    }
}

#[async_trait]
impl Updatable for PaymentLinks {
    type Request = EditPaymentLinkRequest;
    type Response = PaymentLink;

    /// Update payment link details (`PATCH /v1/payment_links/{payment_link_id}`).
    async fn update(
        &self,
        payment_link_id: &str,
        data: Self::Request,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<Self::Response> {
        let path = format!("payment_links/{}", payment_link_id);
        self.http.patch(&path, &data, extra_headers).await
    }
}
