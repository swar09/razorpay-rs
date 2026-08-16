use reqwest::header::HeaderMap;
use std::sync::Arc;

use crate::{
    error::RazorpayResult,
    http::Http,
    models::{ProductConfiguration, TncResponse},
};

/// Resource handle for Razorpay Linked Account Products & TNC API endpoints (`/v2/accounts/{account_id}/products`).
#[derive(Debug, Clone)]
pub struct Products {
    pub(crate) http: Arc<Http>,
}

impl Products {
    pub(crate) fn new(http: Arc<Http>) -> Self {
        Self { http }
    }

    /// Request product configuration for a linked account (`POST /v2/accounts/{account_id}/products`).
    pub async fn request_configuration<T: serde::Serialize + Sync>(
        &self,
        account_id: &str,
        data: &T,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<ProductConfiguration> {
        let path = format!("accounts/{}/products", account_id);
        self.http.post_v2(&path, data, extra_headers).await
    }

    /// Fetch product configuration for a linked account (`GET /v2/accounts/{account_id}/products/{product_id}`).
    pub async fn fetch(
        &self,
        account_id: &str,
        product_id: &str,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<ProductConfiguration> {
        let path = format!("accounts/{}/products/{}", account_id, product_id);
        self.http
            .get_v2::<ProductConfiguration, ()>(&path, None, extra_headers)
            .await
    }

    /// Update product configuration for a linked account (`PATCH /v2/accounts/{account_id}/products/{product_id}`).
    pub async fn update<T: serde::Serialize + Sync>(
        &self,
        account_id: &str,
        product_id: &str,
        data: &T,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<ProductConfiguration> {
        let path = format!("accounts/{}/products/{}", account_id, product_id);
        self.http.patch_v2(&path, data, extra_headers).await
    }

    /// Fetch terms and conditions for a product (`GET /v2/products/{product_name}/tnc`).
    pub async fn fetch_tnc(
        &self,
        product_name: &str,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<TncResponse> {
        let path = format!("products/{}/tnc", product_name);
        self.http
            .get_v2::<TncResponse, ()>(&path, None, extra_headers)
            .await
    }
}
