use reqwest::header::HeaderMap;
use serde::{Serialize, de::DeserializeOwned};
use url::Url;

use crate::{
    config::RazorpayConfig,
    error::{RazorpayError, RazorpayResult},
    models::RazorpayErrorResponse,
};

/// Internal HTTP client wrapper around `reqwest::Client`.
/// Manages basic auth credentials, base URL joining, and response error parsing.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub(crate) struct Http {
    pub(crate) client: reqwest::Client,
    pub(crate) config: RazorpayConfig,
    pub(crate) default_headers: HeaderMap,
}

/// Target API version for Razorpay endpoints.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub(crate) enum ApiVersion {
    V1,
    V2,
}

#[allow(dead_code)]
impl Http {
    pub(crate) fn new(config: RazorpayConfig) -> RazorpayResult<Self> {
        let client = reqwest::Client::builder().timeout(config.timeout).build()?;
        Ok(Self {
            client,
            config,
            default_headers: HeaderMap::new(),
        })
    }

    pub(crate) fn with_client(config: RazorpayConfig, client: reqwest::Client) -> Self {
        Self {
            client,
            config,
            default_headers: HeaderMap::new(),
        }
    }

    pub(crate) fn with_account_id(&self, account_id: &str) -> RazorpayResult<Self> {
        let mut default_headers = self.default_headers.clone();
        let header_value = reqwest::header::HeaderValue::from_str(account_id)
            .map_err(|_| RazorpayError::InvalidInput("invalid account_id for header".into()))?;
        default_headers.insert(
            reqwest::header::HeaderName::from_static("x-razorpay-account"),
            header_value,
        );
        Ok(Self {
            client: self.client.clone(),
            config: self.config.clone(),
            default_headers,
        })
    }

    pub(crate) fn build_versioned_url(
        &self,
        version: ApiVersion,
        path: &str,
    ) -> RazorpayResult<Url> {
        let clean_path = path.trim_start_matches('/');
        let ver = match version {
            ApiVersion::V1 => "v1",
            ApiVersion::V2 => "v2",
        };
        let mut base = self.config.base_url.clone();
        let base_path = base
            .path()
            .trim_end_matches('/')
            .trim_end_matches("/v1")
            .trim_end_matches("/v2");
        let new_path = if base_path.is_empty() {
            format!("/{}/{}", ver, clean_path)
        } else {
            format!("{}/{}/{}", base_path, ver, clean_path)
        };
        base.set_path(&new_path);
        Ok(base)
    }

    fn build_url(&self, path: &str) -> RazorpayResult<Url> {
        if let Some(rest) = path
            .strip_prefix("../v2/")
            .or_else(|| path.strip_prefix("v2/"))
        {
            return self.build_versioned_url(ApiVersion::V2, rest);
        }
        let clean_path = path.trim_start_matches('/');
        let mut base = self.config.base_url.clone();
        if !base.path().ends_with('/') {
            let mut new_path = base.path().to_string();
            new_path.push('/');
            base.set_path(&new_path);
        }
        base.join(clean_path).map_err(RazorpayError::Url)
    }

    async fn handle_response<T: DeserializeOwned>(
        &self,
        response: reqwest::Response,
    ) -> RazorpayResult<T> {
        let status = response.status();
        if status.is_success() {
            let body = response.json::<T>().await?;
            Ok(body)
        } else {
            // Attempt to parse Razorpay API error envelope
            if let Ok(error_response) = response.json::<RazorpayErrorResponse>().await {
                Err(error_response.error.into())
            } else {
                Err(RazorpayError::Api(Box::new(crate::models::RazorpayError {
                    code: format!("HTTP_{}", status.as_u16()),
                    description: format!("Request failed with status {}", status),
                    field: None,
                    source: None,
                    step: None,
                    reason: None,
                    metadata: None,
                })))
            }
        }
    }

    pub(crate) async fn get<T: DeserializeOwned, Q: Serialize + Sync>(
        &self,
        path: &str,
        query: Option<&Q>,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<T> {
        let url = self.build_url(path)?;
        let mut req = self
            .client
            .get(url)
            .basic_auth(&self.config.key_id, Some(&self.config.key_secret));

        if !self.default_headers.is_empty() {
            req = req.headers(self.default_headers.clone());
        }
        if let Some(q) = query {
            req = req.query(q);
        }
        if let Some(headers) = extra_headers {
            req = req.headers(headers);
        }

        let resp = req.send().await?;
        self.handle_response(resp).await
    }

    pub(crate) async fn post<B: Serialize + Sync, T: DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<T> {
        let url = self.build_url(path)?;
        let mut req = self
            .client
            .post(url)
            .basic_auth(&self.config.key_id, Some(&self.config.key_secret))
            .json(body);

        if !self.default_headers.is_empty() {
            req = req.headers(self.default_headers.clone());
        }
        if let Some(headers) = extra_headers {
            req = req.headers(headers);
        }

        let resp = req.send().await?;
        self.handle_response(resp).await
    }

    pub(crate) async fn patch<B: Serialize + Sync, T: DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<T> {
        let url = self.build_url(path)?;
        let mut req = self
            .client
            .patch(url)
            .basic_auth(&self.config.key_id, Some(&self.config.key_secret))
            .json(body);

        if !self.default_headers.is_empty() {
            req = req.headers(self.default_headers.clone());
        }
        if let Some(headers) = extra_headers {
            req = req.headers(headers);
        }

        let resp = req.send().await?;
        self.handle_response(resp).await
    }

    pub(crate) async fn put<B: Serialize + Sync, T: DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<T> {
        let url = self.build_url(path)?;
        let mut req = self
            .client
            .put(url)
            .basic_auth(&self.config.key_id, Some(&self.config.key_secret))
            .json(body);

        if !self.default_headers.is_empty() {
            req = req.headers(self.default_headers.clone());
        }
        if let Some(headers) = extra_headers {
            req = req.headers(headers);
        }

        let resp = req.send().await?;
        self.handle_response(resp).await
    }

    pub(crate) async fn delete<T: DeserializeOwned>(
        &self,
        path: &str,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<T> {
        let url = self.build_url(path)?;
        let mut req = self
            .client
            .delete(url)
            .basic_auth(&self.config.key_id, Some(&self.config.key_secret));

        if !self.default_headers.is_empty() {
            req = req.headers(self.default_headers.clone());
        }
        if let Some(headers) = extra_headers {
            req = req.headers(headers);
        }

        let resp = req.send().await?;
        self.handle_response(resp).await
    }

    pub(crate) async fn multipart<T: DeserializeOwned>(
        &self,
        path: &str,
        form: reqwest::multipart::Form,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<T> {
        let url = self.build_url(path)?;
        let mut req = self
            .client
            .post(url)
            .basic_auth(&self.config.key_id, Some(&self.config.key_secret))
            .multipart(form);

        if !self.default_headers.is_empty() {
            req = req.headers(self.default_headers.clone());
        }
        if let Some(headers) = extra_headers {
            req = req.headers(headers);
        }

        let resp = req.send().await?;
        self.handle_response(resp).await
    }

    pub(crate) async fn get_v2<T: DeserializeOwned, Q: Serialize + Sync>(
        &self,
        path: &str,
        query: Option<&Q>,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<T> {
        let url = self.build_versioned_url(ApiVersion::V2, path)?;
        let mut req = self
            .client
            .get(url)
            .basic_auth(&self.config.key_id, Some(&self.config.key_secret));

        if !self.default_headers.is_empty() {
            req = req.headers(self.default_headers.clone());
        }
        if let Some(q) = query {
            req = req.query(q);
        }
        if let Some(headers) = extra_headers {
            req = req.headers(headers);
        }

        let resp = req.send().await?;
        self.handle_response(resp).await
    }

    pub(crate) async fn post_v2<B: Serialize + Sync, T: DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<T> {
        let url = self.build_versioned_url(ApiVersion::V2, path)?;
        let mut req = self
            .client
            .post(url)
            .basic_auth(&self.config.key_id, Some(&self.config.key_secret))
            .json(body);

        if !self.default_headers.is_empty() {
            req = req.headers(self.default_headers.clone());
        }
        if let Some(headers) = extra_headers {
            req = req.headers(headers);
        }

        let resp = req.send().await?;
        self.handle_response(resp).await
    }

    pub(crate) async fn patch_v2<B: Serialize + Sync, T: DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<T> {
        let url = self.build_versioned_url(ApiVersion::V2, path)?;
        let mut req = self
            .client
            .patch(url)
            .basic_auth(&self.config.key_id, Some(&self.config.key_secret))
            .json(body);

        if !self.default_headers.is_empty() {
            req = req.headers(self.default_headers.clone());
        }
        if let Some(headers) = extra_headers {
            req = req.headers(headers);
        }

        let resp = req.send().await?;
        self.handle_response(resp).await
    }

    pub(crate) async fn delete_v2<T: DeserializeOwned>(
        &self,
        path: &str,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<T> {
        let url = self.build_versioned_url(ApiVersion::V2, path)?;
        let mut req = self
            .client
            .delete(url)
            .basic_auth(&self.config.key_id, Some(&self.config.key_secret));

        if !self.default_headers.is_empty() {
            req = req.headers(self.default_headers.clone());
        }
        if let Some(headers) = extra_headers {
            req = req.headers(headers);
        }

        let resp = req.send().await?;
        self.handle_response(resp).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use std::time::Duration;
    use wiremock::{
        Mock, MockServer, ResponseTemplate,
        matchers::{basic_auth, body_json, header, method, path, query_param},
    };

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct MockEntity {
        id: String,
        amount: u32,
    }

    #[derive(Debug, Serialize)]
    struct MockQuery {
        count: u32,
        skip: u32,
    }

    fn create_test_http(server_uri: &str) -> Http {
        let config = RazorpayConfig {
            key_id: "rzp_test_key".to_string(),
            key_secret: "test_secret".to_string(),
            base_url: Url::parse(server_uri).unwrap(),
            timeout: Duration::from_secs(5),
        };
        Http::new(config).unwrap()
    }

    #[tokio::test]
    async fn test_http_get_with_auth_and_query() {
        let mock_server = MockServer::start().await;
        let http = create_test_http(&mock_server.uri());

        let expected_response = MockEntity {
            id: "order_123".to_string(),
            amount: 500,
        };

        Mock::given(method("GET"))
            .and(path("/orders"))
            .and(basic_auth("rzp_test_key", "test_secret"))
            .and(query_param("count", "10"))
            .and(query_param("skip", "20"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_response))
            .mount(&mock_server)
            .await;

        let query = MockQuery {
            count: 10,
            skip: 20,
        };
        let res: MockEntity = http
            .get("orders", Some(&query), None)
            .await
            .expect("GET request should succeed");

        assert_eq!(res, expected_response);
    }

    #[tokio::test]
    async fn test_http_post_with_json_body() {
        let mock_server = MockServer::start().await;
        let http = create_test_http(&mock_server.uri());

        let payload = MockEntity {
            id: "order_new".to_string(),
            amount: 1000,
        };

        Mock::given(method("POST"))
            .and(path("/orders"))
            .and(basic_auth("rzp_test_key", "test_secret"))
            .and(body_json(&payload))
            .respond_with(ResponseTemplate::new(200).set_body_json(&payload))
            .mount(&mock_server)
            .await;

        let res: MockEntity = http
            .post("orders", &payload, None)
            .await
            .expect("POST request should succeed");

        assert_eq!(res, payload);
    }

    #[tokio::test]
    async fn test_http_patch_with_extra_headers() {
        let mock_server = MockServer::start().await;
        let http = create_test_http(&mock_server.uri());

        let payload = MockEntity {
            id: "order_edit".to_string(),
            amount: 2000,
        };

        Mock::given(method("PATCH"))
            .and(path("/orders/order_edit"))
            .and(header("X-Custom-Header", "test-val"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&payload))
            .mount(&mock_server)
            .await;

        let mut headers = HeaderMap::new();
        headers.insert(
            reqwest::header::HeaderName::from_static("x-custom-header"),
            reqwest::header::HeaderValue::from_static("test-val"),
        );

        let res: MockEntity = http
            .patch("orders/order_edit", &payload, Some(headers))
            .await
            .expect("PATCH request should succeed");

        assert_eq!(res, payload);
    }

    #[tokio::test]
    async fn test_http_delete_success() {
        let mock_server = MockServer::start().await;
        let http = create_test_http(&mock_server.uri());

        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct DeleteResp {
            deleted: bool,
        }

        Mock::given(method("DELETE"))
            .and(path("/items/item_123"))
            .respond_with(ResponseTemplate::new(200).set_body_json(DeleteResp { deleted: true }))
            .mount(&mock_server)
            .await;

        let res: DeleteResp = http
            .delete("items/item_123", None)
            .await
            .expect("DELETE request should succeed");

        assert!(res.deleted);
    }

    #[tokio::test]
    async fn test_http_api_error_envelope_parsing() {
        let mock_server = MockServer::start().await;
        let http = create_test_http(&mock_server.uri());

        let error_body = serde_json::json!({
            "error": {
                "code": "BAD_REQUEST_ERROR",
                "description": "Order amount exceeds maximum limit",
                "field": "amount",
                "source": "business",
                "step": "payment_initiation",
                "reason": "input_validation_failed"
            }
        });

        Mock::given(method("POST"))
            .and(path("/orders"))
            .respond_with(ResponseTemplate::new(400).set_body_json(error_body))
            .mount(&mock_server)
            .await;

        let payload = MockEntity {
            id: "order_err".to_string(),
            amount: 99999999,
        };

        let result: RazorpayResult<MockEntity> = http.post("orders", &payload, None).await;

        match result {
            Err(RazorpayError::Api(api_err)) => {
                assert_eq!(api_err.code, "BAD_REQUEST_ERROR");
                assert_eq!(api_err.description, "Order amount exceeds maximum limit");
                assert_eq!(api_err.field, Some("amount".to_string()));
                assert_eq!(api_err.source, Some("business".to_string()));
                assert_eq!(api_err.step, Some("payment_initiation".to_string()));
                assert_eq!(api_err.reason, Some("input_validation_failed".to_string()));
            }
            other => panic!("Expected RazorpayError::Api, got {:?}", other),
        }
    }

    #[tokio::test]
    async fn test_http_api_error_fallback_when_non_json() {
        let mock_server = MockServer::start().await;
        let http = create_test_http(&mock_server.uri());

        Mock::given(method("GET"))
            .and(path("/broken"))
            .respond_with(ResponseTemplate::new(502).set_body_string("Bad Gateway Server Error"))
            .mount(&mock_server)
            .await;

        let result: RazorpayResult<MockEntity> =
            http.get::<MockEntity, ()>("broken", None, None).await;

        match result {
            Err(RazorpayError::Api(api_err)) => {
                assert_eq!(api_err.code, "HTTP_502");
                assert!(api_err.description.contains("502"));
            }
            other => panic!("Expected RazorpayError::Api fallback, got {:?}", other),
        }
    }
}
