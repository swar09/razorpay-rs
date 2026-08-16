use async_trait::async_trait;
use reqwest::header::HeaderMap;
use std::sync::Arc;

use crate::{
    error::RazorpayResult,
    http::Http,
    models::{ListOptions, RazorpayList, Stakeholder},
    traits::{Fetchable, Listable},
};

/// Stakeholder operations scoped to a specific linked account (`/v2/accounts/{account_id}/stakeholders`).
#[derive(Debug, Clone)]
pub struct Stakeholders {
    pub(crate) http: Arc<Http>,
    pub(crate) account_id: String,
}

impl Stakeholders {
    pub(crate) fn new(http: Arc<Http>, account_id: impl Into<String>) -> Self {
        Self {
            http,
            account_id: account_id.into(),
        }
    }

    /// Create a stakeholder on this linked account (`POST /v2/accounts/{account_id}/stakeholders`).
    pub async fn create<T: serde::Serialize + Send + Sync>(
        &self,
        data: &T,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<Stakeholder> {
        let path = format!("accounts/{}/stakeholders", self.account_id);
        self.http.post_v2(&path, data, extra_headers).await
    }

    /// Update a stakeholder on this linked account (`PATCH /v2/accounts/{account_id}/stakeholders/{stakeholder_id}`).
    pub async fn update<T: serde::Serialize + Send + Sync>(
        &self,
        stakeholder_id: &str,
        data: &T,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<Stakeholder> {
        let path = format!(
            "accounts/{}/stakeholders/{}",
            self.account_id, stakeholder_id
        );
        self.http.patch_v2(&path, data, extra_headers).await
    }
}

#[async_trait]
impl Fetchable for Stakeholders {
    type Item = Stakeholder;

    /// Fetch a stakeholder by ID (`GET /v2/accounts/{account_id}/stakeholders/{stakeholder_id}`).
    async fn fetch(
        &self,
        stakeholder_id: &str,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<Self::Item> {
        let path = format!(
            "accounts/{}/stakeholders/{}",
            self.account_id, stakeholder_id
        );
        self.http
            .get_v2::<Self::Item, ()>(&path, None, extra_headers)
            .await
    }
}

#[async_trait]
impl Listable for Stakeholders {
    type Item = Stakeholder;

    /// Fetch all stakeholders for this account (`GET /v2/accounts/{account_id}/stakeholders`).
    async fn all(
        &self,
        query: Option<ListOptions>,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<RazorpayList<Self::Item>> {
        let path = format!("accounts/{}/stakeholders", self.account_id);
        self.http.get_v2(&path, query.as_ref(), extra_headers).await
    }
}
