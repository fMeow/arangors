#![allow(unused_imports)]
#![allow(unused_parens)]

use log::trace;
use pretty_assertions::assert_eq;
use serde_json::{json, Value};

use crate::common::{collection, connection};
use arangors::index::Index;
use arangors::{
    collection::{
        options::{ChecksumOptions, PropertiesOptions},
        response::Status,
        CollectionType,
    },
    ClientError, Connection, Document,
};
use common::{get_arangodb_host, get_normal_password, get_normal_user, test_setup};

pub mod common;

#[maybe_async::test(
    any(feature = "reqwest_blocking"),
    async(any(feature = "reqwest_async"), tokio::test),
    async(any(feature = "surf_async"), async_std::test)
)]
async fn test_persistent_index() {
    test_setup();
    let collection_name = "test_collection";
    let index_name = "idx_persistent_test";
    let conn = connection().await;

    let mut database = conn.db("test_db").await.unwrap();

    let index = Index::persistent(vec!["password".to_string()], true, false, false)
        .with_name(index_name)
        .create_in_background();

    let result = database
        .create_index(collection_name, &index)
        .await
        .unwrap();

    let id = result.id().as_ref().unwrap();

    // Delete the previously created index
    let delete_result = database.delete_index(&id).await.unwrap();

    assert!(result.id().is_some());
    assert_eq!(result.unique(), &Some(true));
    assert_eq!(result.sparse(), &Some(false));
    assert_eq!(result.deduplicate(), &Some(false));
    assert_eq!(result.name(), &Some(index_name.to_string()));
    assert_eq!(&delete_result.id, id);
}

#[maybe_async::test(
    any(feature = "reqwest_blocking"),
    async(any(feature = "reqwest_async"), tokio::test),
    async(any(feature = "surf_async"), async_std::test)
)]
async fn test_hash_index() {
    test_setup();
    let collection_name = "test_collection";
    let index_name = "idx_hash_test";
    let conn = connection().await;

    let mut database = conn.db("test_db").await.unwrap();

    let index = Index::hash(vec!["password".to_string()], true, false, false)
        .with_name(index_name)
        .create_in_background();

    let result = database
        .create_index(collection_name, &index)
        .await
        .unwrap();

    let id = result.id().as_ref().unwrap();

    // Delete the previously created index
    let delete_result = database.delete_index(&id).await.unwrap();

    assert!(result.id().is_some());
    assert_eq!(result.unique(), &Some(true));
    assert_eq!(result.sparse(), &Some(false));
    assert_eq!(result.deduplicate(), &Some(false));
    assert_eq!(result.name(), &Some(index_name.to_string()));
    assert_eq!(&delete_result.id, id);
}

#[maybe_async::test(
    any(feature = "reqwest_blocking"),
    async(any(feature = "reqwest_async"), tokio::test),
    async(any(feature = "surf_async"), async_std::test)
)]
async fn test_skiplist_index() {
    test_setup();
    let collection_name = "test_collection";
    let index_name = "idx_skiplist_test";
    let conn = connection().await;

    let mut database = conn.db("test_db").await.unwrap();

    let index = Index::skip_list(vec!["password".to_string()], true, false, false)
        .with_name(index_name)
        .create_in_background();

    let result = database
        .create_index(collection_name, &index)
        .await
        .unwrap();

    let id = result.id().as_ref().unwrap();

    // Delete the previously created index
    let delete_result = database.delete_index(&id).await.unwrap();

    assert!(result.id().is_some());
    assert_eq!(result.unique(), &Some(true));
    assert_eq!(result.sparse(), &Some(false));
    assert_eq!(result.deduplicate(), &Some(false));
    assert_eq!(result.name(), &Some(index_name.to_string()));
    assert_eq!(&delete_result.id, id);
}

#[maybe_async::test(
    any(feature = "reqwest_blocking"),
    async(any(feature = "reqwest_async"), tokio::test),
    async(any(feature = "surf_async"), async_std::test)
)]
async fn test_ttl_index() {
    test_setup();
    let collection_name = "test_collection";
    let index_name = "idx_ttl_test";
    let expire_after = 500;
    let conn = connection().await;

    let mut database = conn.db("test_db").await.unwrap();

    let index = Index::ttl(vec!["password".to_string()], expire_after)
        .with_name(index_name)
        .create_in_background();

    let result = database
        .create_index(collection_name, &index)
        .await
        .unwrap();

    let id = result.id().as_ref().unwrap();

    // Delete the previously created index
    let delete_result = database.delete_index(&id).await.unwrap();

    assert!(result.id().is_some());
    assert_eq!(result.unique(), &Some(false));
    assert_eq!(result.sparse(), &Some(true));
    assert!(result.deduplicate().is_none());
    assert_eq!(result.expire_after(), &Some(expire_after));
    assert_eq!(result.name(), &Some(index_name.to_string()));
    assert_eq!(&delete_result.id, id);
}
