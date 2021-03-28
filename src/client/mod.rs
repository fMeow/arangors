//! HTTP client
//!
//! Feature gates are used to conditionally enable specific http ecosystem.
//! Currently reqwest(both blocking and async) and surf(async only) are
//! supported out of the box.
//!
//! But it's possible to incorporate custom ecosystem. See
//! `examples/custom_client.rs`.
use http::{HeaderMap, Request, Response};
use url::Url;

use crate::ClientError;

#[cfg(all(feature = "reqwest_async", feature = "reqwest_blocking"))]
compile_error!(
    r#"feature "reqwest_async" and "reqwest_blocking" cannot be set at the same time. 
If what you want is "reqwest_blocking", please turn off default features by adding "default-features=false" in your Cargo.toml"#
);

#[cfg(all(feature = "reqwest_async", feature = "surf_async"))]
compile_error!(
    r#"feature "reqwest_async" and "surf_async" cannot be set at the same time. 
If what you want is "surf_async", please turn off default features by adding "default-features=false" in your Cargo.toml"#
);

#[cfg(all(
    feature = "reqwest_async",
    feature = "reqwest_blocking",
    feature = "surf_async"
))]
compile_error!(
    r#"only one of features "reqwest_async", "reqwest_blocking" and "surf_async" can be"#
);

#[cfg(any(feature = "reqwest_async", feature = "reqwest_blocking"))]
pub mod reqwest;
#[cfg(any(feature = "surf_async"))]
pub mod surf;

#[maybe_async::maybe_async]
pub trait ClientExt: Sync + Clone {
    fn new<U: Into<Option<HeaderMap>>>(headers: U) -> Result<Self, ClientError>
    where
        Self: Sized;

    fn clone_with_transaction(&self, transaction_id: String) -> Result<Self, ClientError>
    where
        Self: Sized;

    #[inline]
    async fn get<T>(&self, url: Url, text: T) -> Result<Response<String>, ClientError>
    where
        Self: Sized,
        T: Into<String> + Send,
    {
        self.request(Request::get(url.to_string()).body(text.into()).unwrap())
            .await
    }
    #[inline]
    async fn post<T>(&self, url: Url, text: T) -> Result<Response<String>, ClientError>
    where
        Self: Sized,
        T: Into<String> + Send,
    {
        self.request(Request::post(url.to_string()).body(text.into()).unwrap())
            .await
    }
    #[inline]
    async fn put<T>(&self, url: Url, text: T) -> Result<Response<String>, ClientError>
    where
        Self: Sized,
        T: Into<String> + Send,
    {
        self.request(Request::put(url.to_string()).body(text.into()).unwrap())
            .await
    }
    #[inline]
    async fn delete<T>(&self, url: Url, text: T) -> Result<Response<String>, ClientError>
    where
        Self: Sized,
        T: Into<String> + Send,
    {
        self.request(Request::delete(url.to_string()).body(text.into()).unwrap())
            .await
    }
    #[inline]
    async fn patch<T>(&self, url: Url, text: T) -> Result<Response<String>, ClientError>
    where
        Self: Sized,
        T: Into<String> + Send,
    {
        self.request(Request::patch(url.to_string()).body(text.into()).unwrap())
            .await
    }

    #[inline]
    async fn connect<T>(&self, url: Url, text: T) -> Result<Response<String>, ClientError>
    where
        Self: Sized,
        T: Into<String> + Send,
    {
        self.request(Request::connect(url.to_string()).body(text.into()).unwrap())
            .await
    }

    #[inline]
    async fn head<T>(&self, url: Url, text: T) -> Result<Response<String>, ClientError>
    where
        Self: Sized,
        T: Into<String> + Send,
    {
        self.request(Request::head(url.to_string()).body(text.into()).unwrap())
            .await
    }

    #[inline]
    async fn options<T>(&self, url: Url, text: T) -> Result<Response<String>, ClientError>
    where
        Self: Sized,
        T: Into<String> + Send,
    {
        self.request(Request::options(url.to_string()).body(text.into()).unwrap())
            .await
    }

    #[inline]
    async fn trace<T>(&self, url: Url, text: T) -> Result<Response<String>, ClientError>
    where
        Self: Sized,
        T: Into<String> + Send,
    {
        self.request(Request::trace(url.to_string()).body(text.into()).unwrap())
            .await
    }

    async fn request(&self, request: Request<String>) -> Result<Response<String>, ClientError>
    where
        Self: Sized;
}
