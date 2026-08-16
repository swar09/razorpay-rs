use async_trait::async_trait;
use reqwest::header::HeaderMap;
use std::sync::Arc;

use crate::{
    error::RazorpayResult,
    http::Http,
    models::{DeleteResponse, ListOptions, RazorpayList, Token},
    traits::{Deletable, Fetchable, Listable},
};

/// Resource handle for Razorpay Tokens API endpoints (`/v1/tokens` and `/v1/customers/{customer_id}/tokens`).
#[derive(Debug, Clone)]
pub struct Tokens {
    pub(crate) http: Arc<Http>,
}

impl Tokens {
    pub(crate) fn new(http: Arc<Http>) -> Self {
        Self { http }
    }

    /// Fetch a token for a specific customer (`GET /v1/customers/{customer_id}/tokens/{token_id}`).
    pub async fn fetch_for_customer(
        &self,
        customer_id: &str,
        token_id: &str,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<Token> {
        let path = format!("customers/{}/tokens/{}", customer_id, token_id);
        self.http.get::<Token, ()>(&path, None, extra_headers).await
    }

    /// Fetch all tokens for a customer (`GET /v1/customers/{customer_id}/tokens`).
    pub async fn all_for_customer(
        &self,
        customer_id: &str,
        query: Option<ListOptions>,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<RazorpayList<Token>> {
        let path = format!("customers/{}/tokens", customer_id);
        self.http.get(&path, query.as_ref(), extra_headers).await
    }

    /// Delete a customer token (`DELETE /v1/customers/{customer_id}/tokens/{token_id}`).
    pub async fn delete_for_customer(
        &self,
        customer_id: &str,
        token_id: &str,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<DeleteResponse> {
        let path = format!("customers/{}/tokens/{}", customer_id, token_id);
        self.http.delete(&path, extra_headers).await
    }
}

#[async_trait]
impl Fetchable for Tokens {
    type Item = Token;

    /// Fetch a token by ID (`GET /v1/tokens/{token_id}`).
    async fn fetch(&self, token_id: &str, extra_headers: Option<HeaderMap>) -> RazorpayResult<Self::Item> {
        let path = format!("tokens/{}", token_id);
        self.http.get::<Self::Item, ()>(&path, None, extra_headers).await
    }
}

#[async_trait]
impl Listable for Tokens {
    type Item = Token;

    /// Fetch all tokens (`GET /v1/tokens`).
    async fn all(
        &self,
        query: Option<ListOptions>,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<RazorpayList<Self::Item>> {
        self.http.get("tokens", query.as_ref(), extra_headers).await
    }
}

#[async_trait]
impl Deletable for Tokens {
    type Response = DeleteResponse;

    /// Delete a token (`DELETE /v1/tokens/{token_id}`).
    async fn delete(&self, token_id: &str, extra_headers: Option<HeaderMap>) -> RazorpayResult<Self::Response> {
        let path = format!("tokens/{}", token_id);
        self.http.delete(&path, extra_headers).await
    }
}
