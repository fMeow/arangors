#[cfg(feature = "reqwest_blocking")]
use ::reqwest::blocking::{Client, Request};
#[cfg(feature = "reqwest_async")]
use ::reqwest::{Client, Request};

use http::header::HeaderMap;
use url::Url;

use crate::client::ClientExt;

use super::*;
use std::convert::TryFrom;

#[derive(Debug, Clone)]
pub struct ReqwestClient(pub Client);

#[maybe_async::maybe_async]
impl ClientExt for ReqwestClient {
    fn new<U: Into<Option<HeaderMap>>>(headers: U) -> Result<Self, ClientError> {
        let client = Client::builder().gzip(true);
        match headers.into() {
            Some(h) => client.default_headers(h),
            None => client,
        }
        .build()
        .map(|c| ReqwestClient(c))
        .map_err(|e| ClientError::HttpClient(format!("{:?}", e)))
    }

    async fn request(&self, request: http::Request<String>) -> Result<ClientResponse, ClientError> {
        let req: Request = request.into();

        let resp = self
            .0
            .execute(req)
            .await
            .map_err(|e| ClientError::HttpClient(format!("{:?}", e)))?;

        let status_code = resp.status();
        let headers = resp.headers().clone();
        let version = Some(resp.version());
        let content = resp
            .text()
            .await
            .map_err(|e| ClientError::HttpClient(format!("{:?}", e)))?;

        Ok(ClientResponse {
            status_code,
            headers,
            version,
            content,
        })
    }
}
