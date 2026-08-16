use async_trait::async_trait;
use reqwest::header::HeaderMap;
use std::sync::Arc;

use crate::{
    error::RazorpayResult,
    http::Http,
    models::{
        Addon, CreateAddonRequest, CreateSubscriptionRequest, ListOptions, RazorpayList,
        Subscription, UpdateSubscriptionRequest,
    },
    traits::{Creatable, Fetchable, Listable, Updatable},
};

/// Resource handle for Razorpay Subscriptions API endpoints (`/v1/subscriptions`).
#[derive(Debug, Clone)]
pub struct Subscriptions {
    pub(crate) http: Arc<Http>,
}

impl Subscriptions {
    pub(crate) fn new(http: Arc<Http>) -> Self {
        Self { http }
    }

    /// Cancel a subscription (`POST /v1/subscriptions/{subscription_id}/cancel`).
    pub async fn cancel(
        &self,
        subscription_id: &str,
        cancel_at_cycle_end: bool,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<Subscription> {
        let path = format!("subscriptions/{}/cancel", subscription_id);
        let payload = serde_json::json!({
            "cancel_at_cycle_end": if cancel_at_cycle_end { 1 } else { 0 }
        });
        self.http.post(&path, &payload, extra_headers).await
    }

    /// Create an addon on a subscription (`POST /v1/subscriptions/{subscription_id}/addons`).
    pub async fn create_addon(
        &self,
        subscription_id: &str,
        data: CreateAddonRequest,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<Addon> {
        let path = format!("subscriptions/{}/addons", subscription_id);
        self.http.post(&path, &data, extra_headers).await
    }

    /// Pause a subscription (`POST /v1/subscriptions/{subscription_id}/pause`).
    pub async fn pause(
        &self,
        subscription_id: &str,
        pause_at: Option<&str>,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<Subscription> {
        let path = format!("subscriptions/{}/pause", subscription_id);
        let payload = serde_json::json!({
            "pause_at": pause_at.unwrap_or("now")
        });
        self.http.post(&path, &payload, extra_headers).await
    }

    /// Resume a subscription (`POST /v1/subscriptions/{subscription_id}/resume`).
    pub async fn resume(
        &self,
        subscription_id: &str,
        resume_at: Option<&str>,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<Subscription> {
        let path = format!("subscriptions/{}/resume", subscription_id);
        let payload = serde_json::json!({
            "resume_at": resume_at.unwrap_or("now")
        });
        self.http.post(&path, &payload, extra_headers).await
    }

    /// Retrieve scheduled changes for a subscription (`GET /v1/subscriptions/{subscription_id}/retrieve_scheduled_changes`).
    pub async fn pending_update(
        &self,
        subscription_id: &str,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<serde_json::Value> {
        let path = format!("subscriptions/{}/retrieve_scheduled_changes", subscription_id);
        self.http.get::<serde_json::Value, ()>(&path, None, extra_headers).await
    }

    /// Cancel scheduled changes for a subscription (`POST /v1/subscriptions/{subscription_id}/cancel_scheduled_changes`).
    pub async fn cancel_scheduled_changes(
        &self,
        subscription_id: &str,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<Subscription> {
        let path = format!("subscriptions/{}/cancel_scheduled_changes", subscription_id);
        self.http.post(&path, &serde_json::json!({}), extra_headers).await
    }

    /// Delete an offer linked to a subscription (`DELETE /v1/subscriptions/{subscription_id}/{offer_id}`).
    pub async fn delete_offer(
        &self,
        subscription_id: &str,
        offer_id: &str,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<Subscription> {
        let path = format!("subscriptions/{}/{}", subscription_id, offer_id);
        self.http.delete(&path, extra_headers).await
    }
}

#[async_trait]
impl Creatable for Subscriptions {
    type Request = CreateSubscriptionRequest;
    type Response = Subscription;

    /// Create a recurring subscription (`POST /v1/subscriptions`).
    async fn create(&self, data: Self::Request, extra_headers: Option<HeaderMap>) -> RazorpayResult<Self::Response> {
        self.http.post("subscriptions", &data, extra_headers).await
    }
}

#[async_trait]
impl Fetchable for Subscriptions {
    type Item = Subscription;

    /// Fetch a subscription by ID (`GET /v1/subscriptions/{subscription_id}`).
    async fn fetch(&self, subscription_id: &str, extra_headers: Option<HeaderMap>) -> RazorpayResult<Self::Item> {
        let path = format!("subscriptions/{}", subscription_id);
        self.http.get::<Self::Item, ()>(&path, None, extra_headers).await
    }
}

#[async_trait]
impl Listable for Subscriptions {
    type Item = Subscription;

    /// Fetch a paginated collection of subscriptions (`GET /v1/subscriptions`).
    async fn all(
        &self,
        query: Option<ListOptions>,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<RazorpayList<Self::Item>> {
        self.http.get("subscriptions", query.as_ref(), extra_headers).await
    }
}

#[async_trait]
impl Updatable for Subscriptions {
    type Request = UpdateSubscriptionRequest;
    type Response = Subscription;

    /// Update an existing subscription (`PATCH /v1/subscriptions/{subscription_id}`).
    async fn update(
        &self,
        subscription_id: &str,
        data: Self::Request,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<Self::Response> {
        let path = format!("subscriptions/{}", subscription_id);
        self.http.patch(&path, &data, extra_headers).await
    }
}
