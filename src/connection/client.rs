#[cfg(feature = "blocking")]
use reqwest::blocking::Client;
#[cfg(not(feature = "blocking"))]
use reqwest::Client;

use crate::error::HttpError;
use crate::ClientError;
use http::header::HeaderMap;
use http::request::Parts;
use http::{Request, Response};
use reqwest::redirect::Policy;
use std::io::{BufReader, Read};

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
            .redirect(Policy::none())
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
    pub async fn request(&self, request: Request<String>) -> Result<Response<String>, ClientError> {
        self.request_bytes(request.map(|b| b.into_bytes())).await
    }

    #[maybe_async::maybe_async]
    pub async fn request_bytes(
        &self,
        request: Request<Vec<u8>>,
    ) -> Result<Response<String>, ClientError> {
        let req = request.map(|b| BufReader::new(std::io::Cursor::new(b)));
        let res = self.request_reader(req).await?;
        Ok(res)
    }

    #[maybe_async::maybe_async]
    pub async fn request_reader<T>(
        &self,
        mut request: Request<T>,
    ) -> Result<Response<String>, HttpError>
    where
        T: Read + Send + Sync + 'static,
    {
        let headers = request.headers_mut();
        for (header, value) in self.headers.iter() {
            if !headers.contains_key(header) {
                headers.insert(header, value.clone());
            }
        }

        let req = get_req(request);
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

        build
            .status(status_code)
            .version(version)
            .body(content)
            .map_err(|e| HttpError::HttpClient(format!("{:?}", e)))
    }
}

#[maybe_async::async_impl]
fn get_req<T>(req: Request<T>) -> reqwest::Request
where
    T: Read + Send + Sync + 'static,
{
    use futures::StreamExt;
    use reqwest::Body;

    let (parts, body) = req.into_parts();
    let Parts {
        method,
        uri,
        headers,
        ..
    } = parts;

    let mut request = reqwest::Request::new(method, uri.to_string().parse().unwrap());

    let mut prev_name = None;
    for (key, value) in headers {
        match key {
            Some(key) => {
                request.headers_mut().insert(key.clone(), value);
                prev_name = Some(key);
            }
            None => match prev_name {
                Some(ref key) => {
                    request.headers_mut().append(key.clone(), value);
                }
                None => unreachable!("HeaderMap::into_iter yielded None first"),
            },
        }
    }
    let body_bytes = body.bytes();
    let stream = futures::stream::iter(body_bytes).chunks(2048).map(|x| {
        let len = x.len();
        let out = x.into_iter().filter_map(|b| b.ok()).collect::<Vec<_>>();
        if out.len() == len {
            Ok(out)
        } else {
            Err(HttpError::PayloadError)
        }
    });
    request.body_mut().replace(Body::wrap_stream(stream));

    request
}

#[maybe_async::sync_impl]
fn get_req<T>(req: Request<T>) -> reqwest::blocking::Request
where
    T: Read + Send + Sync + 'static,
{
    use reqwest::blocking::Body;

    let (parts, body) = req.into_parts();
    let Parts {
        method,
        uri,
        headers,
        ..
    } = parts;
    let mut request = reqwest::blocking::Request::new(method, uri.to_string().parse().unwrap());

    let mut prev_name = None;
    for (key, value) in headers {
        match key {
            Some(key) => {
                request.headers_mut().insert(key.clone(), value);
                prev_name = Some(key);
            }
            None => match prev_name {
                Some(ref key) => {
                    request.headers_mut().append(key.clone(), value);
                }
                None => unreachable!("HeaderMap::into_iter yielded None first"),
            },
        }
    }
    request.body_mut().replace(Body::new(body));

    request
}
