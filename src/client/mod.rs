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
        self.request(Method::GET, url, text, None).await
    }
    #[inline]
    async fn post(&self, url: Url, text: &str) -> Result<ClientResponse, ClientError>
    where
        Self: Sized,
    {
        self.request(Method::POST, url, text, None).await
    }
    #[inline]
    async fn put(&self, url: Url, text: &str) -> Result<ClientResponse, ClientError>
    where
        Self: Sized,
    {
        self.request(Method::PUT, url, text, None).await
    }
    #[inline]
    async fn delete(&self, url: Url, text: &str) -> Result<ClientResponse, ClientError>
    where
        Self: Sized,
    {
        self.request(Method::DELETE, url, text, None).await
    }
    #[inline]
    async fn patch(&self, url: Url, text: &str) -> Result<ClientResponse, ClientError>
    where
        Self: Sized,
    {
        self.request(Method::PATCH, url, text, None).await
    }

    #[inline]
    async fn connect(&self, url: Url, text: &str) -> Result<ClientResponse, ClientError>
    where
        Self: Sized,
    {
        self.request(Method::CONNECT, url, text, None).await
    }

    #[inline]
    async fn head(&self, url: Url, text: &str) -> Result<ClientResponse, ClientError>
    where
        Self: Sized,
    {
        self.request(Method::HEAD, url, text, None).await
    }

    #[inline]
    async fn options(&self, url: Url, text: &str) -> Result<ClientResponse, ClientError>
    where
        Self: Sized,
    {
        self.request(Method::OPTIONS, url, text, None).await
    }

    #[inline]
    async fn trace(&self, url: Url, text: &str) -> Result<ClientResponse, ClientError>
    where
        Self: Sized,
    {
        self.request(Method::TRACE, url, text, None).await
    }

    #[inline]
    fn build_request(&self, method: Method, url: Url, text: &str) -> ClientRequestBuilder<Self>
    where
        Self: Sized,
    {
        ClientRequestBuilder::new(self.clone(), method, url, text)
    }

    async fn request(
        &self,
        method: Method,
        url: Url,
        text: &str,
        header: Option<RequestHeader>,
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

/// Request header for ClientRequestBuilder
#[derive(Debug)]
pub struct RequestHeader {
    pub key: String,
    pub value: String,
}

#[derive(Debug)]
pub struct ClientRequestBuilder<T>
where
    T: ClientExt,
{
    header: Option<RequestHeader>,
    method: Method,
    url: Url,
    text: String,
    client: T,
}

impl<T> ClientRequestBuilder<T>
where
    T: ClientExt,
{
    pub fn new(client: T, method: Method, url: Url, text: &str) -> ClientRequestBuilder<T> {
        ClientRequestBuilder {
            client,
            header: None,
            method,
            url,
            text: text.to_string(),
        }
    }

    /// Set custom header for one request
    /// If "" injected, it will be ignored
    pub fn set_header(mut self, key: &str, value: &str) -> ClientRequestBuilder<T> {
        if !key.is_empty() && !value.is_empty() {
            self.header = Some(RequestHeader {
                key: key.to_string(),
                value: value.to_string(),
            });
        }
        self
    }

    pub async fn send(self) -> Result<ClientResponse, ClientError>
    where
        Self: Sized,
    {
        self.client
            .request(self.method, self.url, self.text.as_str(), self.header)
            .await
    }
}
