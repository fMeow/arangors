#![allow(unused_imports)]
#![allow(unused_parens)]
use pretty_assertions::assert_eq;

use arangors::{client::ClientExt, connection::Permission, Connection};
use common::{
    get_arangodb_host, get_normal_password, get_normal_user, test_root_and_normal, test_setup,
};

pub mod common;

#[maybe_async::test(
    any(feature = "reqwest_blocking"),
    async(any(feature = "reqwest_async"), tokio::test),
    async(any(feature = "surf_async"), async_std::test)
)]
async fn test_list_databases() {
    test_setup();
    let host = get_arangodb_host();
    let user = get_normal_user();
    let password = get_normal_password();

    let conn = Connection::establish_jwt(&host, &user, &password)
        .await
        .unwrap();
    let dbs = conn.accessible_databases().await.unwrap();

    assert_eq!(dbs.contains_key("test_db"), true);
    let db_permission = dbs.get("test_db").unwrap();
    match db_permission {
        Permission::ReadOnly | Permission::NoAccess => {
            assert!(false, "Invalid permission {:?}", db_permission)
        }
        _ => {}
    };
}

#[maybe_async::test(
    any(feature = "reqwest_blocking"),
    async(any(feature = "reqwest_async"), tokio::test),
    async(any(feature = "surf_async"), async_std::test)
)]
async fn test_get_url() {
    test_setup();
    let host = get_arangodb_host();
    let user = get_normal_user();
    let password = get_normal_password();

    let conn = Connection::establish_jwt(&host, &user, &password)
        .await
        .unwrap();
    let url = conn.get_url().clone().into_string();
    assert_eq!(url, host)
}

#[maybe_async::test(
    any(feature = "reqwest_blocking"),
    async(any(feature = "reqwest_async"), tokio::test),
    async(any(feature = "surf_async"), async_std::test)
)]
async fn test_get_database() {
    test_setup();
    let host = get_arangodb_host();
    let user = get_normal_user();
    let password = get_normal_password();

    let conn = Connection::establish_jwt(&host, &user, &password)
        .await
        .unwrap();
    let database = conn.db("test_db").await;
    assert_eq!(database.is_err(), false);
    let database = conn.db("test_db_non_exist").await;
    assert_eq!(database.is_err(), true);
}

#[maybe_async::test(
    any(feature = "reqwest_blocking"),
    async(any(feature = "reqwest_async"), tokio::test),
    async(any(feature = "surf_async"), async_std::test)
)]
async fn test_basic_auth() {
    test_setup();
    let host = get_arangodb_host();
    let user = get_normal_user();
    let password = get_normal_password();

    let conn = Connection::establish_jwt(&host, &user, &password)
        .await
        .unwrap();
    let session = conn.get_session();
    let resp = session.get(host.parse().unwrap(), "").await.unwrap();
    let headers = resp.headers();
    assert_eq!(headers.get("Server").unwrap(), "ArangoDB");
}

#[maybe_async::test(
    any(feature = "reqwest_blocking"),
    async(any(feature = "reqwest_async"), tokio::test),
    async(any(feature = "surf_async"), async_std::test)
)]
async fn test_jwt() {
    test_setup();
    #[maybe_async::maybe_async]
    async fn jwt(user: String, passwd: String) {
        let host = get_arangodb_host();
        let conn = Connection::establish_jwt(&host, &user, &passwd)
            .await
            .unwrap();
        let session = conn.get_session();
        let resp = session.get(host.parse().unwrap(), "").await.unwrap();
        let headers = resp.headers();
        assert_eq!(headers.get("Server").unwrap(), "ArangoDB");
    }
    test_root_and_normal(jwt).await;
}
