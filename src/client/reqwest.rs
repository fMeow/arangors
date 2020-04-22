#[cfg(feature = "reqwest_blocking")]
use ::reqwest::blocking::Client;
#[cfg(feature = "reqwest_async")]
use ::reqwest::Client;
use http::header::HeaderMap;
use url::Url;

use crate::client::ClientExt;

use super::*;

#[derive(Debug)]
pub struct ReqwestClient(pub Client);

#[maybe_async::maybe_async]
impl ClientExt for ReqwestClient {
    async fn request(&self, method: Method, url: Url, text: &str) -> Result<ClientResponse, Error> {
        let resp = self
            .0
            .request(method, url)
            .body(text.to_owned())
            .send()
            .await?;

        let status_code = resp.status();
        let headers = resp.headers().clone();
        let version = Some(resp.version());
        let content = resp.text().await?;

        Ok(ClientResponse {
            status_code,
            headers,
            version,
            content,
        })
    }

    fn new<U: Into<Option<HeaderMap>>>(headers: U) -> Result<Self, Error> {
        let client = Client::builder().gzip(true);
        match headers.into() {
            Some(h) => client.default_headers(h),
            None => client,
        }
        .build()
        .map(|c| ReqwestClient(c))
        .map_err(Error::from)
    }
}
