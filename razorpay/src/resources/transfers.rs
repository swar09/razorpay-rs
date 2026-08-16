use async_trait::async_trait;
use reqwest::header::HeaderMap;
use std::sync::Arc;

use crate::{
    error::RazorpayResult,
    http::Http,
    models::{
        EditTransferRequest, ListOptions, RazorpayList, ReverseTransferRequest, Transfer,
        TransferRequest, TransferReversal,
    },
    traits::{Creatable, Fetchable, Listable, Updatable},
};

/// Resource handle for Razorpay Transfers (Route) API endpoints (`/v1/transfers`).
#[derive(Debug, Clone)]
pub struct Transfers {
    pub(crate) http: Arc<Http>,
}

impl Transfers {
    pub(crate) fn new(http: Arc<Http>) -> Self {
        Self { http }
    }

    /// Reverse a transfer (`POST /v1/transfers/{transfer_id}/reversals`).
    pub async fn reverse(
        &self,
        transfer_id: &str,
        data: ReverseTransferRequest,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<TransferReversal> {
        let path = format!("transfers/{}/reversals", transfer_id);
        self.http.post(&path, &data, extra_headers).await
    }

    /// Fetch all reversals for a transfer (`GET /v1/transfers/{transfer_id}/reversals`).
    pub async fn reversals(
        &self,
        transfer_id: &str,
        query: Option<ListOptions>,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<RazorpayList<TransferReversal>> {
        let path = format!("transfers/{}/reversals", transfer_id);
        self.http.get(&path, query.as_ref(), extra_headers).await
    }
}

#[async_trait]
impl Creatable for Transfers {
    type Request = TransferRequest;
    type Response = Transfer;

    /// Create a direct transfer (`POST /v1/transfers`).
    async fn create(
        &self,
        data: Self::Request,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<Self::Response> {
        self.http.post("transfers", &data, extra_headers).await
    }
}

#[async_trait]
impl Fetchable for Transfers {
    type Item = Transfer;

    /// Fetch a transfer by ID (`GET /v1/transfers/{transfer_id}`).
    async fn fetch(
        &self,
        transfer_id: &str,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<Self::Item> {
        let path = format!("transfers/{}", transfer_id);
        self.http
            .get::<Self::Item, ()>(&path, None, extra_headers)
            .await
    }
}

#[async_trait]
impl Listable for Transfers {
    type Item = Transfer;

    /// Fetch all transfers (`GET /v1/transfers`).
    async fn all(
        &self,
        query: Option<ListOptions>,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<RazorpayList<Self::Item>> {
        self.http
            .get("transfers", query.as_ref(), extra_headers)
            .await
    }
}

#[async_trait]
impl Updatable for Transfers {
    type Request = EditTransferRequest;
    type Response = Transfer;

    /// Update transfer hold status (`PATCH /v1/transfers/{transfer_id}`).
    async fn update(
        &self,
        transfer_id: &str,
        data: Self::Request,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<Self::Response> {
        let path = format!("transfers/{}", transfer_id);
        self.http.patch(&path, &data, extra_headers).await
    }
}
