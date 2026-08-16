use async_trait::async_trait;
use reqwest::header::HeaderMap;
use std::sync::Arc;

use crate::{
    error::RazorpayResult,
    http::Http,
    models::{
        CreateVirtualAccountReceivers, CreateVirtualAccountRequest, ListOptions, Payment,
        RazorpayList, VirtualAccount,
    },
    traits::{Creatable, Fetchable, Listable},
};

/// Resource handle for Razorpay Virtual Accounts (Smart Collect) API endpoints (`/v1/virtual_accounts`).
#[derive(Debug, Clone)]
pub struct VirtualAccounts {
    pub(crate) http: Arc<Http>,
}

impl VirtualAccounts {
    pub(crate) fn new(http: Arc<Http>) -> Self {
        Self { http }
    }

    /// Close a virtual account (`POST /v1/virtual_accounts/{va_id}/close`).
    pub async fn close(
        &self,
        va_id: &str,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<VirtualAccount> {
        let path = format!("virtual_accounts/{}/close", va_id);
        self.http
            .post(&path, &serde_json::json!({}), extra_headers)
            .await
    }

    /// Fetch all payments received on a virtual account (`GET /v1/virtual_accounts/{va_id}/payments`).
    pub async fn payments(
        &self,
        va_id: &str,
        query: Option<ListOptions>,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<RazorpayList<Payment>> {
        let path = format!("virtual_accounts/{}/payments", va_id);
        self.http.get(&path, query.as_ref(), extra_headers).await
    }

    /// Add receivers to an existing virtual account (`POST /v1/virtual_accounts/{va_id}/receivers`).
    pub async fn add_receiver(
        &self,
        va_id: &str,
        data: CreateVirtualAccountReceivers,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<VirtualAccount> {
        let path = format!("virtual_accounts/{}/receivers", va_id);
        self.http.post(&path, &data, extra_headers).await
    }

    /// Add allowed payers account details (`POST /v1/virtual_accounts/{va_id}/allowed_payers`).
    pub async fn add_allowed_payer<T: serde::Serialize + Sync>(
        &self,
        va_id: &str,
        data: &T,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<serde_json::Value> {
        let path = format!("virtual_accounts/{}/allowed_payers", va_id);
        self.http.post(&path, data, extra_headers).await
    }

    /// Delete allowed payer (`DELETE /v1/virtual_accounts/{va_id}/allowed_payers/{payer_id}`).
    pub async fn delete_allowed_payer(
        &self,
        va_id: &str,
        payer_id: &str,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<crate::models::DeleteResponse> {
        let path = format!("virtual_accounts/{}/allowed_payers/{}", va_id, payer_id);
        self.http.delete(&path, extra_headers).await
    }
}

#[async_trait]
impl Creatable for VirtualAccounts {
    type Request = CreateVirtualAccountRequest;
    type Response = VirtualAccount;

    /// Create a virtual account (`POST /v1/virtual_accounts`).
    async fn create(
        &self,
        data: Self::Request,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<Self::Response> {
        self.http
            .post("virtual_accounts", &data, extra_headers)
            .await
    }
}

#[async_trait]
impl Fetchable for VirtualAccounts {
    type Item = VirtualAccount;

    /// Fetch a virtual account by ID (`GET /v1/virtual_accounts/{va_id}`).
    async fn fetch(
        &self,
        va_id: &str,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<Self::Item> {
        let path = format!("virtual_accounts/{}", va_id);
        self.http
            .get::<Self::Item, ()>(&path, None, extra_headers)
            .await
    }
}

#[async_trait]
impl Listable for VirtualAccounts {
    type Item = VirtualAccount;

    /// Fetch all virtual accounts (`GET /v1/virtual_accounts`).
    async fn all(
        &self,
        query: Option<ListOptions>,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<RazorpayList<Self::Item>> {
        self.http
            .get("virtual_accounts", query.as_ref(), extra_headers)
            .await
    }
}
