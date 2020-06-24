#![allow(unused_imports)]
#![allow(unused_parens)]
use log::trace;
use pretty_assertions::assert_eq;

use arangors::Connection;
use common::{
    get_arangodb_host, get_normal_password, get_normal_user, get_root_password, get_root_user,
    test_root_and_normal, test_setup,
};

pub mod common;

const NEW_DB_NAME: &str = "example";

#[maybe_async::test(
    any(feature = "reqwest_blocking"),
    async(any(feature = "reqwest_async"), tokio::test),
    async(any(feature = "surf_async"), async_std::test)
)]
async fn test_create_and_drop_database() {
    test_setup();
    let host = get_arangodb_host();
    let root_user = get_root_user();
    let root_password = get_root_password();

    let conn = Connection::establish_jwt(&host, &root_user, &root_password)
        .await
        .unwrap();

    let result = conn.create_database(NEW_DB_NAME).await;
    if let Err(e) = result {
        assert!(false, "Fail to create database: {:?}", e)
    };
    let result = conn.db(NEW_DB_NAME).await;
    assert_eq!(result.is_err(), false);

    let result = conn.drop_database(NEW_DB_NAME).await;
    if let Err(e) = result {
        assert!(false, "Fail to drop database: {:?}", e)
    };
    let result = conn.db(NEW_DB_NAME).await;
    assert_eq!(result.is_err(), true);
}

#[maybe_async::test(
    any(feature = "reqwest_blocking"),
    async(any(feature = "reqwest_async"), tokio::test),
    async(any(feature = "surf_async"), async_std::test)
)]
async fn test_fetch_current_database_info() {
    test_setup();

    #[maybe_async::maybe_async]
    async fn fetch_current_database(user: String, passwd: String) {
        let host = get_arangodb_host();
        let conn = Connection::establish_jwt(&host, &user, &passwd)
            .await
            .unwrap();
        let db = conn.db("test_db").await.unwrap();
        let info = db.info().await;
        match info {
            Ok(info) => {
                trace!("{:?}", info);
                assert_eq!(info.is_system, false)
            }
            Err(e) => assert!(false, "Fail to fetch database: {:?}", e),
        }
    }
    test_root_and_normal(fetch_current_database).await;
}

#[maybe_async::test(
    any(feature = "reqwest_blocking"),
    async(any(feature = "reqwest_async"), tokio::test),
    async(any(feature = "surf_async"), async_std::test)
)]
async fn test_get_version() {
    test_setup();
    let host = get_arangodb_host();
    let user = get_normal_user();
    let password = get_normal_password();

    let conn = Connection::establish_jwt(&host, &user, &password)
        .await
        .unwrap();
    let db = conn.db("test_db").await.unwrap();
    let version = db.arango_version().await.unwrap();
    trace!("{:?}", version);
    assert_eq!(version.license, "community");
    assert_eq!(version.server, "arango");

    let re = regex::Regex::new(r"3\.\d\.\d+").unwrap();
    assert_eq!(re.is_match(&version.version), true);
}
