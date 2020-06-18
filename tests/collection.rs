#![allow(unused_imports)]
#![allow(unused_parens)]

use log::trace;
use pretty_assertions::assert_eq;

use arangors::{ClientError, Connection};
use common::{get_arangodb_host, get_normal_password, get_normal_user, test_setup};

pub mod common;

#[maybe_async::test(
    sync = r#"any(feature="reqwest_blocking")"#,
    async = r#"any(feature="reqwest_async")"#,
    test = "tokio::test"
)]
#[cfg_attr(feature = "surf_async", maybe_async::must_be_async, async_std::test)]
async fn test_get_collection() {
    test_setup();
    let host = get_arangodb_host();
    let user = get_normal_user();
    let password = get_normal_password();

    let conn = Connection::establish_jwt(&host, &user, &password)
        .await
        .unwrap();
    let database = conn.db("test_db").await.unwrap();
    let coll = database.accessible_collections().await;
    trace!("{:?}", coll);
    let coll = database.collection("test_collection").await;
    assert_eq!(coll.is_err(), false);
    let coll = database.collection("test_collection_non_exists").await;
    assert_eq!(coll.is_err(), true);
}

#[maybe_async::test(
    sync = r#"any(feature="reqwest_blocking")"#,
    async = r#"any(feature="reqwest_async")"#,
    test = "tokio::test"
)]
#[cfg_attr(feature = "surf_async", maybe_async::must_be_async, async_std::test)]
async fn test_create_and_drop_collection() {
    test_setup();
    let host = get_arangodb_host();
    let user = get_normal_user();
    let password = get_normal_password();

    let collection_name = "test_collection_create_and_drop";

    let conn = Connection::establish_jwt(&host, &user, &password)
        .await
        .unwrap();
    let mut database = conn.db("test_db").await.unwrap();

    let coll = database.drop_collection(collection_name).await;
    assert_eq!(coll.is_err(), true);
    let coll = database.create_collection(collection_name).await;
    assert_eq!(coll.is_err(), false);
    let coll = database.drop_collection(collection_name).await;
    assert_eq!(coll.is_err(), false);
}

#[maybe_async::test(
    sync = r#"any(feature="reqwest_blocking")"#,
    async = r#"any(feature="reqwest_async")"#,
    test = "tokio::test"
)]
#[cfg_attr(feature = "surf_async", maybe_async::must_be_async, async_std::test)]
async fn test_get_properties() {
    test_setup();
    let host = get_arangodb_host();
    let user = get_normal_user();
    let password = get_normal_password();

    let collection_name = "test_collection_properties";

    let conn = Connection::establish_jwt(&host, &user, &password)
        .await
        .unwrap();
    let mut database = conn.db("test_db").await.unwrap();

    let coll = database.drop_collection(collection_name).await;
    assert_eq!(coll.is_err(), true);

    let coll = database.create_collection(collection_name).await;
    assert_eq!(coll.is_err(), false);

    let coll = database.collection(collection_name).await;
    assert_eq!(coll.is_err(), false);

    let properties = coll.unwrap().properties().await;
    assert_eq!(properties.is_err(), false);

    let result = properties.unwrap();

    assert_eq!(result.name, collection_name);
    assert_eq!(result.cache_enabled, false);
    assert_eq!(result.is_system, false);
    assert_eq!(result.wait_for_sync, false);
    assert_eq!(result.key_options.allow_user_keys, true);
    assert_eq!(result.key_options.r#type, Some("traditional".to_string()));
    assert_eq!(result.key_options.last_value, Some(0));
    assert_eq!(result.status, 3);
    assert_eq!(result.write_concern, 1);

    let coll = database.drop_collection(collection_name).await;
    assert_eq!(coll.is_err(), false);
}
