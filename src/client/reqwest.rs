//! Reqwest HTTP client
use std::convert::TryInto;

#[cfg(any(feature = "reqwest_blocking"))]
use ::reqwest::blocking::Client;

#[cfg(any(feature = "reqwest_async"))]
use ::reqwest::Client;

use http::header::HeaderMap;

use super::ClientExt;
use crate::ClientError;
use http::HeaderValue;

#[derive(Debug, Clone)]
pub struct ReqwestClient {
    pub client: Client,
    headers: HeaderMap,
}

#[maybe_async::maybe_async]
impl ClientExt for ReqwestClient {
    fn new<U: Into<Option<HeaderMap>>>(headers: U) -> Result<Self, ClientError> {
        let client = Client::builder().gzip(true);
        let headers = match headers.into() {
            Some(h) => h,
            None => HeaderMap::new(),
        };

        client
            .build()
            .map(|c| ReqwestClient { client: c, headers })
            .map_err(|e| ClientError::HttpClient(format!("{:?}", e)))
    }

    fn headers(&mut self) -> &mut HeaderMap<HeaderValue> {
        &mut self.headers
    }

    async fn request(
        &self,
        mut request: http::Request<String>,
    ) -> Result<http::Response<String>, ClientError> {
        let headers = request.headers_mut();
        for (header, value) in self.headers.iter() {
            if !headers.contains_key(header) {
                headers.insert(header, value.clone());
            }
        }
        let req = request.try_into().unwrap();

        let resp = self
            .client
            .execute(req)
            .await
            .map_err(|e| ClientError::HttpClient(format!("{:?}", e)))?;

        let status_code = resp.status();
        let headers = resp.headers().clone();
        let version = resp.version();
        let content = resp
            .text()
            .await
            .map_err(|e| ClientError::HttpClient(format!("{:?}", e)))?;
        let mut build = http::Response::builder();

        for header in headers.iter() {
            build = build.header(header.0, header.1);
        }

        build
            .status(status_code)
            .version(version)
            .body(content)
            .map_err(|e| ClientError::HttpClient(format!("{:?}", e)))
    }
}
