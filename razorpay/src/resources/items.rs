use async_trait::async_trait;
use reqwest::header::HeaderMap;
use std::sync::Arc;

use crate::{
    error::RazorpayResult,
    http::Http,
    models::{
        CreateItemRequest, DeleteResponse, Item, ListOptions, RazorpayList, UpdateItemRequest,
    },
    traits::{Creatable, Deletable, Fetchable, Listable, Updatable},
};

/// Resource handle for Razorpay Items API endpoints (`/v1/items`).
#[derive(Debug, Clone)]
pub struct Items {
    pub(crate) http: Arc<Http>,
}

impl Items {
    pub(crate) fn new(http: Arc<Http>) -> Self {
        Self { http }
    }
}

#[async_trait]
impl Creatable for Items {
    type Request = CreateItemRequest;
    type Response = Item;

    /// Create an item (`POST /v1/items`).
    async fn create(
        &self,
        data: Self::Request,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<Self::Response> {
        self.http.post("items", &data, extra_headers).await
    }
}

#[async_trait]
impl Fetchable for Items {
    type Item = Item;

    /// Fetch an item by ID (`GET /v1/items/{item_id}`).
    async fn fetch(
        &self,
        item_id: &str,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<Self::Item> {
        let path = format!("items/{}", item_id);
        self.http
            .get::<Self::Item, ()>(&path, None, extra_headers)
            .await
    }
}

#[async_trait]
impl Listable for Items {
    type Item = Item;

    /// Fetch all items (`GET /v1/items`).
    async fn all(
        &self,
        query: Option<ListOptions>,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<RazorpayList<Self::Item>> {
        self.http.get("items", query.as_ref(), extra_headers).await
    }
}

#[async_trait]
impl Updatable for Items {
    type Request = UpdateItemRequest;
    type Response = Item;

    /// Update an item (`PATCH /v1/items/{item_id}`).
    async fn update(
        &self,
        item_id: &str,
        data: Self::Request,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<Self::Response> {
        let path = format!("items/{}", item_id);
        self.http.patch(&path, &data, extra_headers).await
    }
}

#[async_trait]
impl Deletable for Items {
    type Response = DeleteResponse;

    /// Delete an item (`DELETE /v1/items/{item_id}`).
    async fn delete(
        &self,
        item_id: &str,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<Self::Response> {
        let path = format!("items/{}", item_id);
        self.http.delete(&path, extra_headers).await
    }
}
