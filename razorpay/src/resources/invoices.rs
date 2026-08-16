use async_trait::async_trait;
use reqwest::header::HeaderMap;
use std::sync::Arc;

use crate::{
    error::RazorpayResult,
    http::Http,
    models::{
        CreateInvoiceRequest, DeleteResponse, EditInvoiceRequest, Invoice, ListOptions,
        NotifyMedium, RazorpayList,
    },
    traits::{Creatable, Deletable, Fetchable, Listable, Updatable},
};

/// Resource handle for Razorpay Invoices API endpoints (`/v1/invoices`).
#[derive(Debug, Clone)]
pub struct Invoices {
    pub(crate) http: Arc<Http>,
}

impl Invoices {
    pub(crate) fn new(http: Arc<Http>) -> Self {
        Self { http }
    }

    /// Issue a draft invoice (`POST /v1/invoices/{invoice_id}/issue`).
    pub async fn issue(&self, invoice_id: &str, extra_headers: Option<HeaderMap>) -> RazorpayResult<Invoice> {
        let path = format!("invoices/{}/issue", invoice_id);
        self.http.post(&path, &serde_json::json!({}), extra_headers).await
    }

    /// Cancel an issued invoice (`POST /v1/invoices/{invoice_id}/cancel`).
    pub async fn cancel(&self, invoice_id: &str, extra_headers: Option<HeaderMap>) -> RazorpayResult<Invoice> {
        let path = format!("invoices/{}/cancel", invoice_id);
        self.http.post(&path, &serde_json::json!({}), extra_headers).await
    }

    /// Send or resend notification for an invoice (`POST /v1/invoices/{invoice_id}/notify_by/{medium}`).
    pub async fn notify_by(
        &self,
        invoice_id: &str,
        medium: NotifyMedium,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<serde_json::Value> {
        let medium_str = match medium {
            NotifyMedium::Sms => "sms",
            NotifyMedium::Email => "email",
        };
        let path = format!("invoices/{}/notify_by/{}", invoice_id, medium_str);
        self.http.post(&path, &serde_json::json!({}), extra_headers).await
    }
}

#[async_trait]
impl Creatable for Invoices {
    type Request = CreateInvoiceRequest;
    type Response = Invoice;

    /// Create an invoice (`POST /v1/invoices`).
    async fn create(&self, data: Self::Request, extra_headers: Option<HeaderMap>) -> RazorpayResult<Self::Response> {
        self.http.post("invoices", &data, extra_headers).await
    }
}

#[async_trait]
impl Fetchable for Invoices {
    type Item = Invoice;

    /// Fetch an invoice by ID (`GET /v1/invoices/{invoice_id}`).
    async fn fetch(&self, invoice_id: &str, extra_headers: Option<HeaderMap>) -> RazorpayResult<Self::Item> {
        let path = format!("invoices/{}", invoice_id);
        self.http.get::<Self::Item, ()>(&path, None, extra_headers).await
    }
}

#[async_trait]
impl Listable for Invoices {
    type Item = Invoice;

    /// Fetch all invoices (`GET /v1/invoices`).
    async fn all(
        &self,
        query: Option<ListOptions>,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<RazorpayList<Self::Item>> {
        self.http.get("invoices", query.as_ref(), extra_headers).await
    }
}

#[async_trait]
impl Updatable for Invoices {
    type Request = EditInvoiceRequest;
    type Response = Invoice;

    /// Update draft invoice details (`PATCH /v1/invoices/{invoice_id}`).
    async fn update(
        &self,
        invoice_id: &str,
        data: Self::Request,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<Self::Response> {
        let path = format!("invoices/{}", invoice_id);
        self.http.patch(&path, &data, extra_headers).await
    }
}

#[async_trait]
impl Deletable for Invoices {
    type Response = DeleteResponse;

    /// Delete a draft invoice (`DELETE /v1/invoices/{invoice_id}`).
    async fn delete(&self, invoice_id: &str, extra_headers: Option<HeaderMap>) -> RazorpayResult<Self::Response> {
        let path = format!("invoices/{}", invoice_id);
        self.http.delete(&path, extra_headers).await
    }
}
