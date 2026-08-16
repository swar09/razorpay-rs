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

    /// Obtain a handle to manage fund account validations (penny drop / UPI verification).
    pub fn validations(&self) -> FundAccountValidations {
        FundAccountValidations::new(Arc::clone(&self.http))
    }

    /// Create a fund account validation (`POST /v1/fund_accounts/validations`).
    pub async fn create_validation(
        &self,
        data: crate::models::CreateFundAccountValidationRequest,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<crate::models::FundAccountValidation> {
        self.validations().create(data, extra_headers).await
    }

    /// Fetch a fund account validation by ID (`GET /v1/fund_accounts/validations/{validation_id}`).
    pub async fn fetch_validation(
        &self,
        validation_id: &str,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<crate::models::FundAccountValidation> {
        self.validations().fetch(validation_id, extra_headers).await
    }

    /// List all fund account validations (`GET /v1/fund_accounts/validations`).
    pub async fn all_validations(
        &self,
        query: Option<ListOptions>,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<RazorpayList<crate::models::FundAccountValidation>> {
        self.validations().all(query, extra_headers).await
    }
}

/// Resource handle for RazorpayX Fund Account Validations (`/v1/fund_accounts/validations`).
#[derive(Debug, Clone)]
pub struct FundAccountValidations {
    pub(crate) http: Arc<Http>,
}

impl FundAccountValidations {
    pub(crate) fn new(http: Arc<Http>) -> Self {
        Self { http }
    }
}

#[async_trait]
impl Creatable for FundAccountValidations {
    type Request = crate::models::CreateFundAccountValidationRequest;
    type Response = crate::models::FundAccountValidation;

    /// Initiate a fund account penny-drop or UPI validation (`POST /v1/fund_accounts/validations`).
    async fn create(
        &self,
        data: Self::Request,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<Self::Response> {
        self.http
            .post("fund_accounts/validations", &data, extra_headers)
            .await
    }
}

#[async_trait]
impl Fetchable for FundAccountValidations {
    type Item = crate::models::FundAccountValidation;

    /// Fetch fund account validation details (`GET /v1/fund_accounts/validations/{validation_id}`).
    async fn fetch(
        &self,
        validation_id: &str,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<Self::Item> {
        let path = format!("fund_accounts/validations/{}", validation_id);
        self.http
            .get::<Self::Item, ()>(&path, None, extra_headers)
            .await
    }
}

#[async_trait]
impl Listable for FundAccountValidations {
    type Item = crate::models::FundAccountValidation;

    /// List fund account validation records (`GET /v1/fund_accounts/validations`).
    async fn all(
        &self,
        query: Option<ListOptions>,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<RazorpayList<Self::Item>> {
        self.http
            .get("fund_accounts/validations", query.as_ref(), extra_headers)
            .await
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
