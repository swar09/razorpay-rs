use async_trait::async_trait;
use reqwest::header::HeaderMap;
use std::sync::Arc;

use crate::{
    error::RazorpayResult,
    http::Http,
    models::{CreateOrderRequest, ListOptions, Order, Payment, RazorpayList},
    traits::{Creatable, Fetchable, Listable},
};

/// Resource handle for Razorpay Orders API endpoints (`/v1/orders`).
#[derive(Debug, Clone)]
pub struct Orders {
    pub(crate) http: Arc<Http>,
}

impl Orders {
    pub(crate) fn new(http: Arc<Http>) -> Self {
        Self { http }
    }

    /// Fetch all payments authorized or captured for a specific order (`GET /v1/orders/{order_id}/payments`).
    pub async fn payments(
        &self,
        order_id: &str,
        query: Option<ListOptions>,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<RazorpayList<Payment>> {
        let path = format!("orders/{}/payments", order_id);
        self.http.get(&path, query.as_ref(), extra_headers).await
    }
}

#[async_trait]
impl Creatable for Orders {
    type Request = CreateOrderRequest;
    type Response = Order;

    /// Create a new order (`POST /v1/orders`).
    async fn create(&self, data: Self::Request, extra_headers: Option<HeaderMap>) -> RazorpayResult<Self::Response> {
        self.http.post("orders", &data, extra_headers).await
    }
}

#[async_trait]
impl Fetchable for Orders {
    type Item = Order;

    /// Fetch an order by its ID (`GET /v1/orders/{order_id}`).
    async fn fetch(&self, order_id: &str, extra_headers: Option<HeaderMap>) -> RazorpayResult<Self::Item> {
        let path = format!("orders/{}", order_id);
        self.http.get::<Self::Item, ()>(&path, None, extra_headers).await
    }
}

#[async_trait]
impl Listable for Orders {
    type Item = Order;

    /// Fetch a paginated list of orders (`GET /v1/orders`).
    async fn all(
        &self,
        query: Option<ListOptions>,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<RazorpayList<Self::Item>> {
        self.http.get("orders", query.as_ref(), extra_headers).await
    }
}
