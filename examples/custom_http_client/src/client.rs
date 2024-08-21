//! Reqwest HTTP client
use std::convert::TryInto;

use ::reqwest::header::HeaderValue;
use ::reqwest::Client;
use http::header::HeaderMap;

use arangors::client::ClientExt;
use arangors::ClientError;

#[derive(Debug, Clone)]
pub struct ReqwestClient {
    pub client: Client,
    pub headers: HeaderMap,
}

#[async_trait::async_trait]
impl ClientExt for ReqwestClient {
    fn headers(&mut self) -> &mut HeaderMap<HeaderValue> {
        &mut self.headers
    }

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
