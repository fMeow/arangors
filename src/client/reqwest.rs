#[cfg(feature = "reqwest_blocking")]
use ::reqwest::blocking::Client;
#[cfg(feature = "reqwest_async")]
use ::reqwest::Client;

use http::header::HeaderMap;

use crate::client::ClientExt;

use super::*;

use std::convert::TryInto;

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

    async fn request(
        &self,
        request: http::Request<String>,
    ) -> Result<http::Response<String>, ClientError> {
        let req = request.try_into().unwrap();

        let resp = self
            .0
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

        http::response::Builder::from(build)
            .status(status_code)
            .version(version)
            .body(content)
            .map_err(|e| ClientError::HttpClient(format!("{:?}", e)))
    }
}
