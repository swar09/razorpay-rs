use reqwest::header::HeaderMap;
use std::sync::Arc;

use crate::{
    error::RazorpayResult,
    http::Http,
    models::{
        CreateWebhookRequest, DeleteResponse, ListOptions, RazorpayList, UpdateWebhookRequest,
        Webhook,
    },
};

/// Resource handle for Razorpay Webhook management endpoints (`/v1/webhooks` and `/v2/accounts/{account_id}/webhooks`).
///
/// Enables programmatic creation, retrieval, modification, and deletion of webhook endpoints.
#[derive(Debug, Clone)]
pub struct Webhooks {
    pub(crate) http: Arc<Http>,
}

impl Webhooks {
    pub(crate) fn new(http: Arc<Http>) -> Self {
        Self { http }
    }

    /// Create a new webhook subscription (`POST /v1/webhooks` or `POST /v2/accounts/{account_id}/webhooks`).
    ///
    /// Pass `account_id = None` to create a standard account webhook, or `Some("acc_xxx")` for a Route linked account.
    pub async fn create(
        &self,
        account_id: Option<&str>,
        data: &CreateWebhookRequest,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<Webhook> {
        match account_id {
            Some(id) => {
                let path = format!("accounts/{}/webhooks", id);
                self.http.post_v2(&path, data, extra_headers).await
            }
            None => self.http.post("webhooks", data, extra_headers).await,
        }
    }

    /// Fetch a webhook subscription by ID (`GET /v1/webhooks/{webhook_id}` or `GET /v2/accounts/{account_id}/webhooks/{webhook_id}`).
    pub async fn fetch(
        &self,
        webhook_id: &str,
        account_id: Option<&str>,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<Webhook> {
        match account_id {
            Some(id) => {
                let path = format!("accounts/{}/webhooks/{}", id, webhook_id);
                self.http
                    .get_v2::<Webhook, ()>(&path, None, extra_headers)
                    .await
            }
            None => {
                let path = format!("webhooks/{}", webhook_id);
                self.http
                    .get::<Webhook, ()>(&path, None, extra_headers)
                    .await
            }
        }
    }

    /// Update a webhook subscription (`PUT /v1/webhooks/{webhook_id}` or `PATCH /v2/accounts/{account_id}/webhooks/{webhook_id}`).
    pub async fn edit(
        &self,
        webhook_id: &str,
        account_id: Option<&str>,
        data: &UpdateWebhookRequest,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<Webhook> {
        match account_id {
            Some(id) => {
                let path = format!("accounts/{}/webhooks/{}", id, webhook_id);
                self.http.patch_v2(&path, data, extra_headers).await
            }
            None => {
                let path = format!("webhooks/{}", webhook_id);
                self.http.put(&path, data, extra_headers).await
            }
        }
    }

    /// List all webhook subscriptions (`GET /v1/webhooks` or `GET /v2/accounts/{account_id}/webhooks`).
    pub async fn all(
        &self,
        account_id: Option<&str>,
        query: Option<ListOptions>,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<RazorpayList<Webhook>> {
        match account_id {
            Some(id) => {
                let path = format!("accounts/{}/webhooks", id);
                self.http.get_v2(&path, query.as_ref(), extra_headers).await
            }
            None => {
                self.http
                    .get("webhooks", query.as_ref(), extra_headers)
                    .await
            }
        }
    }

    /// Delete a webhook subscription (`DELETE /v1/webhooks/{webhook_id}` or `DELETE /v2/accounts/{account_id}/webhooks/{webhook_id}`).
    pub async fn delete(
        &self,
        webhook_id: &str,
        account_id: Option<&str>,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<DeleteResponse> {
        match account_id {
            Some(id) => {
                let path = format!("accounts/{}/webhooks/{}", id, webhook_id);
                self.http.delete_v2(&path, extra_headers).await
            }
            None => {
                let path = format!("webhooks/{}", webhook_id);
                self.http.delete(&path, extra_headers).await
            }
        }
    }
}
