use async_trait::async_trait;
use reqwest::header::HeaderMap;
use std::sync::Arc;

use crate::{
    error::RazorpayResult,
    http::Http,
    models::{DeleteResponse, LinkedAccount},
    resources::stakeholders::Stakeholders,
    traits::{Deletable, Fetchable},
};

/// Resource handle for Razorpay Linked Accounts (v2 Route API endpoints: `/v2/accounts`).
#[derive(Debug, Clone)]
pub struct Accounts {
    pub(crate) http: Arc<Http>,
}

impl Accounts {
    pub(crate) fn new(http: Arc<Http>) -> Self {
        Self { http }
    }

    /// Obtain a handle to manage stakeholders for an account.
    pub fn stakeholders(&self, account_id: impl Into<String>) -> Stakeholders {
        Stakeholders::new(Arc::clone(&self.http), account_id)
    }

    /// Create a linked account (`POST /v2/accounts`).
    pub async fn create<T: serde::Serialize + Send + Sync>(
        &self,
        data: &T,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<LinkedAccount> {
        self.http.post("../v2/accounts", data, extra_headers).await
    }

    /// Edit a linked account (`PATCH /v2/accounts/{account_id}`).
    pub async fn update<T: serde::Serialize + Send + Sync>(
        &self,
        account_id: &str,
        data: &T,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<LinkedAccount> {
        let path = format!("../v2/accounts/{}", account_id);
        self.http.patch(&path, data, extra_headers).await
    }
}

#[async_trait]
impl Fetchable for Accounts {
    type Item = LinkedAccount;

    /// Fetch a linked account by ID (`GET /v2/accounts/{account_id}`).
    async fn fetch(
        &self,
        account_id: &str,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<Self::Item> {
        let path = format!("../v2/accounts/{}", account_id);
        self.http
            .get::<Self::Item, ()>(&path, None, extra_headers)
            .await
    }
}

#[async_trait]
impl Deletable for Accounts {
    type Response = DeleteResponse;

    /// Delete a linked account (`DELETE /v2/accounts/{account_id}`).
    async fn delete(
        &self,
        account_id: &str,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<Self::Response> {
        let path = format!("../v2/accounts/{}", account_id);
        self.http.delete(&path, extra_headers).await
    }
}
