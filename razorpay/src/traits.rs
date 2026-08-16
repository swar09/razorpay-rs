use async_trait::async_trait;
use reqwest::header::HeaderMap;
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    error::RazorpayResult,
    models::{ListOptions, RazorpayList},
};

/// Common trait for resources that support fetching a single entity by ID.
#[async_trait]
pub trait Fetchable {
    /// Deserialized response entity type.
    type Item: DeserializeOwned + Send;

    /// Fetch a resource entity by its primary ID.
    async fn fetch(&self, id: &str, extra_headers: Option<HeaderMap>) -> RazorpayResult<Self::Item>;
}

/// Common trait for resources that support retrieving a paginated collection.
#[async_trait]
pub trait Listable {
    /// Deserialized item type contained in the list.
    type Item: DeserializeOwned + Send;

    /// Fetch a list of entities matching optional pagination/filter criteria.
    async fn all(
        &self,
        query: Option<ListOptions>,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<RazorpayList<Self::Item>>;
}

/// Common trait for resources that support creating a new entity.
#[async_trait]
pub trait Creatable {
    /// Request payload type.
    type Request: Serialize + Send + Sync;
    /// Deserialized created entity response type.
    type Response: DeserializeOwned + Send;

    /// Create a new resource entity with the given payload.
    async fn create(&self, data: Self::Request, extra_headers: Option<HeaderMap>) -> RazorpayResult<Self::Response>;
}

/// Common trait for resources that support modifying an existing entity.
#[async_trait]
pub trait Updatable {
    /// Update payload type.
    type Request: Serialize + Send + Sync;
    /// Deserialized updated entity response type.
    type Response: DeserializeOwned + Send;

    /// Update an existing resource entity by ID with the given payload.
    async fn update(
        &self,
        id: &str,
        data: Self::Request,
        extra_headers: Option<HeaderMap>,
    ) -> RazorpayResult<Self::Response>;
}

/// Common trait for resources that support entity deletion / cancellation.
#[async_trait]
pub trait Deletable {
    /// Deserialized response type (e.g. DeleteResponse or deleted entity).
    type Response: DeserializeOwned + Send;

    /// Delete or cancel a resource entity by its primary ID.
    async fn delete(&self, id: &str, extra_headers: Option<HeaderMap>) -> RazorpayResult<Self::Response>;
}
