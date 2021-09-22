#[cfg(feature = "blocking")]
use reqwest::blocking::Client;
#[cfg(not(feature = "blocking"))]
use reqwest::Client;

#[cfg(feature = "blocking")]
type Reqwest = reqwest::blocking::Request;
#[cfg(not(feature = "blocking"))]
type Reqwest = reqwest::Request;

use crate::error::HttpError;
use crate::ClientError;
use http::header::HeaderMap;
use http::{Request, Response};
use std::convert::TryFrom;

#[derive(Debug, Clone)]
pub struct ReqwestClient {
    pub client: Client,
    headers: HeaderMap,
}

impl ReqwestClient {
    pub fn with_client(
        client: Client,
        headers: impl Into<Option<HeaderMap>>,
    ) -> Result<Self, ClientError> {
        let headers = match headers.into() {
            Some(h) => h,
            None => HeaderMap::new(),
        };
        Ok(Self { client, headers })
    }

    pub fn new(headers: impl Into<Option<HeaderMap>>) -> Result<Self, ClientError> {
        let client = Client::builder().gzip(true);
        let headers = match headers.into() {
            Some(h) => h,
            None => HeaderMap::new(),
        };

        client
            .build()
            .map(|c| ReqwestClient { client: c, headers })
            .map_err(|e| ClientError::HttpClient(HttpError::HttpClient(format!("{:?}", e))))
    }

    pub fn headers(&mut self) -> &mut HeaderMap {
        &mut self.headers
    }

    #[maybe_async::maybe_async]
    #[inline]
    pub async fn get<T>(&self, url: String, text: T) -> Result<Response<String>, ClientError>
    where
        T: Into<String> + Send,
    {
        self.request(Request::get(url).body(text.into()).unwrap())
            .await
    }

    #[maybe_async::maybe_async]
    #[inline]
    pub async fn post<T>(&self, url: String, text: T) -> Result<Response<String>, ClientError>
    where
        T: Into<String> + Send,
    {
        self.request(Request::post(url).body(text.into()).unwrap())
            .await
    }

    #[maybe_async::maybe_async]
    #[inline]
    pub async fn put<T>(&self, url: String, text: T) -> Result<Response<String>, ClientError>
    where
        T: Into<String> + Send,
    {
        self.request(Request::put(url).body(text.into()).unwrap())
            .await
    }

    #[maybe_async::maybe_async]
    #[inline]
    pub async fn delete<T>(&self, url: String, text: T) -> Result<Response<String>, ClientError>
    where
        T: Into<String> + Send,
    {
        self.request(Request::delete(url).body(text.into()).unwrap())
            .await
    }

    #[maybe_async::maybe_async]
    #[inline]
    pub async fn patch<T>(&self, url: String, text: T) -> Result<Response<String>, ClientError>
    where
        T: Into<String> + Send,
    {
        self.request(Request::patch(url).body(text.into()).unwrap())
            .await
    }

    #[maybe_async::maybe_async]
    #[inline]
    pub async fn connect<T>(&self, url: String, text: T) -> Result<Response<String>, ClientError>
    where
        T: Into<String> + Send,
    {
        self.request(Request::connect(url).body(text.into()).unwrap())
            .await
    }

    #[maybe_async::maybe_async]
    #[inline]
    pub async fn head<T>(&self, url: String, text: T) -> Result<Response<String>, ClientError>
    where
        T: Into<String> + Send,
    {
        self.request(Request::head(url).body(text.into()).unwrap())
            .await
    }

    #[maybe_async::maybe_async]
    #[inline]
    pub async fn options<T>(&self, url: String, text: T) -> Result<Response<String>, ClientError>
    where
        T: Into<String> + Send,
    {
        self.request(Request::options(url).body(text.into()).unwrap())
            .await
    }

    #[maybe_async::maybe_async]
    #[inline]
    pub async fn trace<T>(&self, url: String, text: T) -> Result<Response<String>, ClientError>
    where
        T: Into<String> + Send,
    {
        self.request(Request::trace(url).body(text.into()).unwrap())
            .await
    }

    #[maybe_async::maybe_async]
    pub async fn request(
        &self,
        mut request: Request<String>,
    ) -> Result<Response<String>, ClientError> {
        let headers = request.headers_mut();
        for (header, value) in self.headers.iter() {
            if !headers.contains_key(header) {
                headers.insert(header, value.clone());
            }
        }

        let req = Reqwest::try_from(request).unwrap();
        let resp = self
            .client
            .execute(req)
            .await
            .map_err(|e| HttpError::HttpClient(format!("{:?}", e)))?;

        let status_code = resp.status();
        let headers = resp.headers().clone();
        let version = resp.version();
        let content = resp
            .text()
            .await
            .map_err(|e| HttpError::HttpClient(format!("{:?}", e)))?;
        let mut build = http::Response::builder();

        for header in headers.iter() {
            build = build.header(header.0, header.1);
        }

        let res = build
            .status(status_code)
            .version(version)
            .body(content)
            .map_err(|e| HttpError::HttpClient(format!("{:?}", e)))?;
        Ok(res)
    }
}
