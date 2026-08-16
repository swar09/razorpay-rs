use async_trait::async_trait;
use reqwest::header::HeaderMap;
use std::sync::Arc;

use crate::{
    error::RazorpayResult,
    http::Http,
    models::{
        CreateInstantSettlementRequest, InstantSettlement, ListOptions, RazorpayList, Settlement,
        SettlementReconItem,
    },
    traits::{Fetchable, Listable},
};

/// Resource handle for Razorpay Settlements API endpoints (`/v1/settlements`).
#[derive(Debug, Clone)]
pub struct Settlements {
    pub(crate) http: Arc<Http>,
}

impl Settlements {
    pub(crate) fn new(http: Arc<Http>) -> Self {
        Self { http }
    }

    /// Fetch combined settlement reconciliation reports (`GET /v1/settlements/recon/combined`).
    pub async fn reports(
        &self,
        query: Option<ListOptions>,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<RazorpayList<SettlementReconItem>> {
        self.http.get("settlements/recon/combined", query.as_ref(), extra_headers).await
    }

    /// Create an on-demand instant settlement (`POST /v1/settlements/ondemand`).
    pub async fn create_ondemand(
        &self,
        data: CreateInstantSettlementRequest,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<InstantSettlement> {
        self.http.post("settlements/ondemand", &data, extra_headers).await
    }

    /// Fetch all on-demand settlements (`GET /v1/settlements/ondemand`).
    pub async fn all_ondemand(
        &self,
        query: Option<ListOptions>,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<RazorpayList<InstantSettlement>> {
        self.http.get("settlements/ondemand", query.as_ref(), extra_headers).await
    }

    /// Fetch an on-demand settlement by its ID (`GET /v1/settlements/ondemand/{ondemand_id}`).
    pub async fn ondemand_by_id(
        &self,
        ondemand_id: &str,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<InstantSettlement> {
        let path = format!("settlements/ondemand/{}", ondemand_id);
        self.http.get::<InstantSettlement, ()>(&path, None, extra_headers).await
    }
}

#[async_trait]
impl Fetchable for Settlements {
    type Item = Settlement;

    /// Fetch a settlement by ID (`GET /v1/settlements/{settlement_id}`).
    async fn fetch(&self, settlement_id: &str, extra_headers: Option<HeaderMap>) -> RazorpayResult<Self::Item> {
        let path = format!("settlements/{}", settlement_id);
        self.http.get::<Self::Item, ()>(&path, None, extra_headers).await
    }
}

#[async_trait]
impl Listable for Settlements {
    type Item = Settlement;

    /// Fetch all settlements (`GET /v1/settlements`).
    async fn all(
        &self,
        query: Option<ListOptions>,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<RazorpayList<Self::Item>> {
        self.http.get("settlements", query.as_ref(), extra_headers).await
    }
}
