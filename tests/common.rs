#![allow(unused_imports)]
#![allow(unused_parens)]
use arangors::{connection::Connection, Collection, Database};
use std::{env, future::Future};

pub const ARANGODB_HOST: &str = "http://localhost:8529/";

pub const ROOT_USERNAME: &str = "root";
pub const ROOT_PASSWORD: &str = "KWNngteTps7XjrNv";

pub const NORMAL_USERNAME: &str = "username";
pub const NORMAL_PASSWORD: &str = "password";

pub fn get_root_user() -> String {
    env::var("ARANGO_ROOT_USER").unwrap_or(ROOT_USERNAME.to_owned())
}

pub fn get_root_password() -> String {
    env::var("ARANGO_ROOT_PASSWORD").unwrap_or(ROOT_PASSWORD.to_owned())
}

pub fn get_normal_user() -> String {
    env::var("ARANGO_USER").unwrap_or(NORMAL_USERNAME.to_owned())
}

pub fn get_normal_password() -> String {
    env::var("ARANGO_PASSWORD").unwrap_or(NORMAL_PASSWORD.to_owned())
}

pub fn get_arangodb_host() -> String {
    env::var("ARANGODB_HOST")
        .map(|s| format!("http://{}", s))
        .unwrap_or(ARANGODB_HOST.to_owned())
}

#[test]
pub fn test_setup() {
    match env_logger::Builder::from_default_env()
        .is_test(true)
        .try_init()
    {
        _ => {}
    }
}

#[maybe_async::maybe_async]
pub async fn connection() -> arangors::Connection {
    let host = get_arangodb_host();
    let user = get_normal_user();
    let password = get_normal_password();

    Connection::establish_jwt(&host, &user, &password)
        .await
        .unwrap()
}

#[cfg(any(feature = "reqwest_async", feature = "reqwest_blocking"))]
#[maybe_async::maybe_async]
pub async fn collection<'a>(
    conn: &'a arangors::Connection,
    name: &str,
) -> Collection<uclient::reqwest::ReqwestClient> {
    let database = conn.db("test_db").await.unwrap();

    match database.drop_collection(name).await {
        _ => {}
    };
    database
        .create_collection(name)
        .await
        .expect("Fail to create the collection");
    database.collection(name).await.unwrap()
}

#[cfg(feature = "surf_async")]
#[maybe_async::maybe_async]
pub async fn collection<'a>(
    conn: &'a arangors::Connection,
    name: &str,
) -> Collection<uclient::surf::SurfClient> {
    let database = conn.db("test_db").await.unwrap();

    match database.drop_collection(name).await {
        _ => {}
    };

    database
        .create_collection(name)
        .await
        .expect("Fail to create the collection");
    database.collection(name).await.unwrap()
}

#[maybe_async::sync_impl]
pub fn test_root_and_normal<T>(test: T) -> ()
where
    T: Fn(String, String) -> (),
{
    test(get_root_user(), get_root_password());
    test(get_normal_user(), get_normal_password());
}

#[maybe_async::async_impl]
pub async fn test_root_and_normal<T, F>(test: T) -> ()
where
    T: Fn(String, String) -> F,
    F: Future<Output = ()>,
{
    test(get_root_user(), get_root_password()).await;
    test(get_normal_user(), get_normal_password()).await;
}
