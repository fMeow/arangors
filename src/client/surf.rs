use std::str::FromStr;

use http::{
    header::{HeaderMap, HeaderName, HeaderValue, CONTENT_LENGTH, SERVER},
    StatusCode, Version,
};
use url::Url;

use crate::client::ClientExt;

use super::*;

#[derive(Debug, Clone)]
pub struct SurfClient {
    pub headers: HeaderMap,
}

#[async_trait::async_trait]
impl ClientExt for SurfClient {
    fn new<U: Into<Option<HeaderMap>>>(headers: U) -> Result<Self, ClientError> {
        let headers = match headers.into() {
            Some(h) => h,
            None => HeaderMap::new(),
        };

        Ok(SurfClient { headers })
    }

    async fn request(
        &self,
        request: http::Request<String>,
    ) -> Result<http::Response<String>, ClientError> {
        use ::surf::http_types::headers::HeaderName as SurfHeaderName;

        let method = request.method().clone();
        let url = request.uri().to_owned().to_string();
        let text = request.body();
        log::trace!("{:?}({:?}): {} ", method, url, text);

        let req = match method {
            Method::GET => ::surf::get(url),
            Method::POST => ::surf::post(url),
            Method::PUT => ::surf::put(url),
            Method::DELETE => ::surf::delete(url),
            Method::PATCH => ::surf::patch(url),
            Method::CONNECT => ::surf::connect(url),
            Method::HEAD => ::surf::head(url),
            Method::OPTIONS => ::surf::options(url),
            Method::TRACE => ::surf::trace(url),
            m @ _ => return Err(ClientError::HttpClient(format!("invalid method {}", m))),
        };

        let mut req = self.headers.iter().fold(req, |req, (k, v)| {
            req.set_header(
                SurfHeaderName::from_str(k.as_str()).unwrap(),
                v.to_str().unwrap(),
            )
        });
        let copy_req = req;
        let mut req = request.headers().iter().fold(copy_req, |req, (k, v)| {
            req.set_header(
                SurfHeaderName::from_str(k.as_str()).unwrap(),
                v.to_str().unwrap(),
            )
        });

        let mut resp = req
            .body_string(text.to_owned())
            .await
            .map_err(|e| ClientError::HttpClient(format!("{:?}", e)))?;

        let status_code = resp.status();

        let mut headers = HeaderMap::new();
        let conv_header = |hn: HeaderName| -> HeaderValue {
            let v = resp
                .header(&SurfHeaderName::from_str(hn.as_str()).unwrap())
                .unwrap();
            let mut iter = v.iter();
            let acc = iter.next().map(|v| v.as_str()).unwrap_or("").to_owned();
            let s = iter.fold(acc, |acc, x| format!("{};{}", acc, x.as_str()));
            s.parse().unwrap()
        };
        headers.insert(SERVER, conv_header(SERVER));
        headers.insert(CONTENT_LENGTH, conv_header(CONTENT_LENGTH));

        let version = resp.version();
        let content = resp
            .body_string()
            .await
            .map_err(|e| ClientError::HttpClient(format!("{:?}", e)))?;

        let mut build = http::Response::builder();

        for header in headers.iter() {
            build = build.header(header.0, header.1);
        }

        let http_version = version.map(|v| match v {
            http_types::Version::Http0_9 => Version::HTTP_09,
            http_types::Version::Http1_0 => Version::HTTP_10,
            http_types::Version::Http1_1 => Version::HTTP_11,
            http_types::Version::Http2_0 => Version::HTTP_2,
            http_types::Version::Http3_0 => Version::HTTP_3,
            _ => unreachable!(),
        });

        http::response::Builder::from(build)
            .status(status_code)
            .version(http_version.unwrap())
            .body(content)
            .map_err(|e| ClientError::HttpClient(format!("{:?}", e)))
    }
}
