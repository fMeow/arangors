use std::fmt::Debug;

use http::{method::Method, HeaderMap};
use serde::de::DeserializeOwned;
use url::Url;

use crate::ClientError;

#[cfg(any(feature = "reqwest_async", feature = "reqwest_blocking"))]
pub mod reqwest;
#[cfg(any(feature = "surf_async"))]
pub mod surf;

#[maybe_async::maybe_async]
pub trait ClientExt: Sync + Debug + Clone {
    fn new<U: Into<Option<HeaderMap>>>(headers: U) -> Result<Self, ClientError>
    where
        Self: Sized;

    #[inline]
    async fn get(&self, url: Url, text: &str) -> Result<ClientResponse, ClientError>
    where
        Self: Sized,
    {
        self.request(Method::GET, url, text).await
    }
    #[inline]
    async fn post(&self, url: Url, text: &str) -> Result<ClientResponse, ClientError>
    where
        Self: Sized,
    {
        self.request(Method::POST, url, text).await
    }
    #[inline]
    async fn put(&self, url: Url, text: &str) -> Result<ClientResponse, ClientError>
    where
        Self: Sized,
    {
        self.request(Method::PUT, url, text).await
    }
    #[inline]
    async fn delete(&self, url: Url, text: &str) -> Result<ClientResponse, ClientError>
    where
        Self: Sized,
    {
        self.request(Method::DELETE, url, text).await
    }
    #[inline]
    async fn patch(&self, url: Url, text: &str) -> Result<ClientResponse, ClientError>
    where
        Self: Sized,
    {
        self.request(Method::PATCH, url, text).await
    }

    #[inline]
    async fn connect(&self, url: Url, text: &str) -> Result<ClientResponse, ClientError>
    where
        Self: Sized,
    {
        self.request(Method::CONNECT, url, text).await
    }

    #[inline]
    async fn head(&self, url: Url, text: &str) -> Result<ClientResponse, ClientError>
    where
        Self: Sized,
    {
        self.request(Method::HEAD, url, text).await
    }

    #[inline]
    async fn options(&self, url: Url, text: &str) -> Result<ClientResponse, ClientError>
    where
        Self: Sized,
    {
        self.request(Method::OPTIONS, url, text).await
    }

    #[inline]
    async fn trace(&self, url: Url, text: &str) -> Result<ClientResponse, ClientError>
    where
        Self: Sized,
    {
        self.request(Method::TRACE, url, text).await
    }

    async fn request(
        &self,
        method: Method,
        url: Url,
        text: &str,
    ) -> Result<ClientResponse, ClientError>
    where
        Self: Sized;
}

#[derive(Debug)]
pub struct ClientResponse {
    status_code: http::StatusCode,
    headers: http::HeaderMap,
    content: String,
    version: Option<http::Version>,
}

impl ClientResponse {
    pub fn new(
        status_code: http::StatusCode,
        headers: http::HeaderMap,
        content: String,
        version: Option<http::Version>,
    ) -> Self {
        Self {
            status_code,
            headers,
            content,
            version,
        }
    }

    /// Get the `StatusCode` of this `Response`.
    #[inline]
    pub fn status(&self) -> http::StatusCode {
        self.status_code
    }

    /// Get the HTTP `Version` of this `Response`.
    #[inline]
    pub fn version(&self) -> Option<http::Version> {
        self.version
    }

    /// Get the `Headers` of this `Response`.
    #[inline]
    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    /// Get response content in `String`
    #[inline]
    pub fn text(&self) -> &str {
        &self.content
    }

    /// Get response content in `Json`
    #[inline]
    pub fn json<T: DeserializeOwned>(&self) -> Result<T, ClientError> {
        Ok(serde_json::from_str(self.text())?)
    }
}
