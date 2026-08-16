use async_trait::async_trait;
use reqwest::header::HeaderMap;
use std::sync::Arc;

use crate::{
    error::RazorpayResult,
    http::Http,
    models::{
        CreateCustomerRequest, Customer, DeleteResponse, EditCustomerRequest, ListOptions,
        RazorpayList, Token,
    },
    traits::{Creatable, Deletable, Fetchable, Listable, Updatable},
};

/// Resource handle for Razorpay Customers API endpoints (`/v1/customers`).
#[derive(Debug, Clone)]
pub struct Customers {
    pub(crate) http: Arc<Http>,
}

impl Customers {
    pub(crate) fn new(http: Arc<Http>) -> Self {
        Self { http }
    }

    /// Obtain a token handle scoped to a specific customer ID.
    pub fn tokens(&self, customer_id: impl Into<String>) -> CustomerTokens {
        CustomerTokens {
            http: Arc::clone(&self.http),
            customer_id: customer_id.into(),
        }
    }
}

#[async_trait]
impl Creatable for Customers {
    type Request = CreateCustomerRequest;
    type Response = Customer;

    /// Create a customer (`POST /v1/customers`).
    async fn create(&self, data: Self::Request, extra_headers: Option<HeaderMap>) -> RazorpayResult<Self::Response> {
        self.http.post("customers", &data, extra_headers).await
    }
}

#[async_trait]
impl Fetchable for Customers {
    type Item = Customer;

    /// Fetch a customer by ID (`GET /v1/customers/{customer_id}`).
    async fn fetch(&self, customer_id: &str, extra_headers: Option<HeaderMap>) -> RazorpayResult<Self::Item> {
        let path = format!("customers/{}", customer_id);
        self.http.get::<Self::Item, ()>(&path, None, extra_headers).await
    }
}

#[async_trait]
impl Listable for Customers {
    type Item = Customer;

    /// Fetch all customers (`GET /v1/customers`).
    async fn all(
        &self,
        query: Option<ListOptions>,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<RazorpayList<Self::Item>> {
        self.http.get("customers", query.as_ref(), extra_headers).await
    }
}

#[async_trait]
impl Updatable for Customers {
    type Request = EditCustomerRequest;
    type Response = Customer;

    /// Edit customer details (`PUT /v1/customers/{customer_id}`).
    async fn update(
        &self,
        customer_id: &str,
        data: Self::Request,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<Self::Response> {
        let path = format!("customers/{}", customer_id);
        self.http.put(&path, &data, extra_headers).await
    }
}

/// Token operations scoped to a specific customer.
#[derive(Debug, Clone)]
pub struct CustomerTokens {
    pub(crate) http: Arc<Http>,
    pub(crate) customer_id: String,
}

#[async_trait]
impl Fetchable for CustomerTokens {
    type Item = Token;

    /// Fetch a specific customer token (`GET /v1/customers/{customer_id}/tokens/{token_id}`).
    async fn fetch(&self, token_id: &str, extra_headers: Option<HeaderMap>) -> RazorpayResult<Self::Item> {
        let path = format!("customers/{}/tokens/{}", self.customer_id, token_id);
        self.http.get::<Self::Item, ()>(&path, None, extra_headers).await
    }
}

#[async_trait]
impl Listable for CustomerTokens {
    type Item = Token;

    /// Fetch all tokens for this customer (`GET /v1/customers/{customer_id}/tokens`).
    async fn all(
        &self,
        query: Option<ListOptions>,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<RazorpayList<Self::Item>> {
        let path = format!("customers/{}/tokens", self.customer_id);
        self.http.get(&path, query.as_ref(), extra_headers).await
    }
}

#[async_trait]
impl Deletable for CustomerTokens {
    type Response = DeleteResponse;

    /// Delete a customer token (`DELETE /v1/customers/{customer_id}/tokens/{token_id}`).
    async fn delete(&self, token_id: &str, extra_headers: Option<HeaderMap>) -> RazorpayResult<Self::Response> {
        let path = format!("customers/{}/tokens/{}", self.customer_id, token_id);
        self.http.delete(&path, extra_headers).await
    }
}
