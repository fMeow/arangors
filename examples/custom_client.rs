#![allow(unused_imports)]
#![allow(unused_parens)]

//! You might want you stick with certain ecosystem like reqwest or actix-web,
//! and don't want to mingle two ecosystem, since it significantly increase
//! compile time and binary size.
//!
//! Arangors offers you flexibility to use any HTTP client you like for both
//! async and sync implementation. Thanks to maybe_async, we can now unify async
//! and sync API.
//!
//! Several implementations are provided: async `reqwest`, blocking `reqwest`,
//! `surf`(async-std) and later `awc`.
use anyhow::Error;
use http::{HeaderMap, Method};
#[cfg(feature = "reqwest_async")]
use reqwest::Client;
use url::Url;

use arangors::{
    client::{ClientExt, ClientResponse},
    ClientError, GenericConnection,
};

/// when use async http client, `blocking` feature MUST be disabled
// This cfg is only to make rust compiler happy, you can just ignore it
#[cfg(feature = "reqwest_async")]
#[derive(Debug, Clone)]
pub struct ReqwestClient(pub Client);

/// you can also use macro: maybe_async::async_impl, with which the whole code
/// block will just vanish when you enabled `blocking` feature.
/// Also, the API of reqwest is almost the same for async and sync. You can also
/// use maybe_async::maybe_async to remove async/await keyword in need, and just
/// import reqwesat::Client and rewest::blocking::Client respectively in async
/// and sync implementation. See `arangors::client::reqwest` source code.
// This cfg is only to make rust compiler happy, you can just ignore it
#[cfg(feature = "reqwest_async")]
#[async_trait::async_trait]
impl ClientExt for ReqwestClient {
    async fn request(&self, request: http::Request<String>) -> Result<ClientResponse, ClientError> {
        let resp = self
            .0
            .execute(request.into())
            .await
            .map_err(|e| ClientError::HttpClient(format!("{:?}", e)))?;

        let status_code = resp.status();
        let headers = resp.headers().clone();
        let version = Some(resp.version());
        let content = resp
            .text()
            .await
            .map_err(|e| ClientError::HttpClient(format!("{:?}", e)))?;

        Ok(ClientResponse::new(status_code, headers, content, version))
    }

    fn new<U: Into<Option<HeaderMap>>>(headers: U) -> Result<Self, ClientError> {
        match headers.into() {
            Some(h) => Client::builder().default_headers(h),
            None => Client::builder(),
        }
        .build()
        .map(|c| ReqwestClient(c))
        .map_err(|e| ClientError::HttpClient(format!("{:?}", e)))
    }
}

// This cfg is only to make rust compiler happy, you can just ignore it
#[cfg(feature = "reqwest_async")]
#[tokio::main]
async fn main() -> Result<(), Error> {
    const URL: &str = "http://localhost:8529";
    let conn =
        GenericConnection::<ReqwestClient>::establish_jwt(URL, "username", "password").await?;
    // from here the API is the same
    let db = conn.db("test_db").await?;
    let info = db.info().await?;
    println!("{:?}", info);

    Ok(())
}

#[cfg(not(feature = "reqwest_async"))]
fn main() {}
