//! Reqwest HTTP client
use std::convert::TryInto;

#[cfg(feature = "reqwest_blocking")]
use ::reqwest::blocking::Client;
#[cfg(feature = "reqwest_async")]
use ::reqwest::Client;
use http::header::HeaderMap;

use super::*;
use crate::client::ClientExt;
use crate::transaction::TRANSACTION_HEADER;

#[derive(Debug, Clone)]
pub struct ReqwestClient {
    pub client: Client,
    pub headers: HeaderMap,
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
            .default_headers(headers.clone())
            .build()
            .map(|c| ReqwestClient { client: c, headers })
            .map_err(|e| ClientError::HttpClient(format!("{:?}", e)))
    }

    fn clone_with_transaction(&self, transaction_id: String) -> Result<Self, ClientError> {
        let mut headers = HeaderMap::new();
        for (name, value) in self.headers.iter() {
            headers.insert(name, value.clone());
        }

        headers.insert(TRANSACTION_HEADER, transaction_id.parse().unwrap());
        ReqwestClient::new(headers)
    }

    async fn request(
        &self,
        request: http::Request<String>,
    ) -> Result<http::Response<String>, ClientError> {
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
