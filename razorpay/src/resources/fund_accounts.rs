use async_trait::async_trait;
use reqwest::header::HeaderMap;
use std::sync::Arc;

use crate::{
    error::RazorpayResult,
    http::Http,
    models::{CreateFundAccountRequest, FundAccount, ListOptions, RazorpayList},
    traits::{Creatable, Fetchable, Listable},
};

/// Resource handle for RazorpayX Fund Accounts API endpoints (`/v1/fund_accounts`).
#[derive(Debug, Clone)]
pub struct FundAccounts {
    pub(crate) http: Arc<Http>,
}

impl FundAccounts {
    pub(crate) fn new(http: Arc<Http>) -> Self {
        Self { http }
    }
}

#[async_trait]
impl Creatable for FundAccounts {
    type Request = CreateFundAccountRequest;
    type Response = FundAccount;

    /// Create a fund account (`POST /v1/fund_accounts`).
    async fn create(
        &self,
        data: Self::Request,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<Self::Response> {
        self.http.post("fund_accounts", &data, extra_headers).await
    }
}

#[async_trait]
impl Fetchable for FundAccounts {
    type Item = FundAccount;

    /// Fetch a fund account by ID (`GET /v1/fund_accounts/{fund_account_id}`).
    async fn fetch(
        &self,
        fund_account_id: &str,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<Self::Item> {
        let path = format!("fund_accounts/{}", fund_account_id);
        self.http
            .get::<Self::Item, ()>(&path, None, extra_headers)
            .await
    }
}

#[async_trait]
impl Listable for FundAccounts {
    type Item = FundAccount;

    /// Fetch all fund accounts (`GET /v1/fund_accounts`).
    async fn all(
        &self,
        query: Option<ListOptions>,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<RazorpayList<Self::Item>> {
        self.http
            .get("fund_accounts", query.as_ref(), extra_headers)
            .await
    }
}
