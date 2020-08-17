#![allow(unused_imports)]
#![allow(unused_parens)]

use log::trace;
use pretty_assertions::assert_eq;
use serde_json::{json, Value};

use crate::common::{collection, connection};
use arangors::index::{BasicIndex, FulltextIndex, GeoIndex, Index, TtlIndex};
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

    let index = BasicIndex::builder()
        .name(index_name.to_string())
        .fields(vec!["password".to_string()])
        .unique(true)
        .build();

    let index = Index::Persistent(index);

    let result = database
        .create_index(collection_name, &index)
        .await
        .unwrap();

    if let Index::Persistent(index) = result {
        let id = index.id.as_ref().unwrap();
        let delete_result = database.delete_index(&id).await.unwrap();

        assert!(index.id.is_some());
        assert_eq!(index.unique, true);
        assert_eq!(index.sparse, false);
        assert_eq!(index.deduplicate, Some(false));
        assert_eq!(index.name, Some(index_name.to_string()));
        assert_eq!(&delete_result.id, id);
    } else {
        panic!("Wrong index type");
    }
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

    let index = BasicIndex::builder()
        .name(index_name.to_string())
        .fields(vec!["password".to_string()])
        .unique(true)
        .build();

    let index = Index::Hash(index);

    let result = database
        .create_index(collection_name, &index)
        .await
        .unwrap();

    if let Index::Hash(index) = result {
        let id = index.id.as_ref().unwrap();
        let delete_result = database.delete_index(&id).await.unwrap();

        assert!(index.id.is_some());
        assert_eq!(index.unique, true);
        assert_eq!(index.sparse, false);
        assert_eq!(index.deduplicate, Some(false));
        assert_eq!(index.name, Some(index_name.to_string()));
        assert_eq!(&delete_result.id, id);
    } else {
        panic!("Wrong index type");
    }
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

    let index = BasicIndex::builder()
        .name(index_name.to_string())
        .fields(vec!["password".to_string()])
        .unique(true)
        .build();

    let index = Index::Skiplist(index);

    let result = database
        .create_index(collection_name, &index)
        .await
        .unwrap();

    if let Index::Skiplist(index) = result {
        let id = index.id.as_ref().unwrap();
        let delete_result = database.delete_index(&id).await.unwrap();

        assert!(index.id.is_some());
        assert_eq!(index.unique, true);
        assert_eq!(index.sparse, false);
        assert_eq!(index.deduplicate, Some(false));
        assert_eq!(index.name, Some(index_name.to_string()));
        assert_eq!(&delete_result.id, id);
    } else {
        panic!("Wrong index type");
    }
}

#[maybe_async::test(
    any(feature = "reqwest_blocking"),
    async(any(feature = "reqwest_async"), tokio::test),
    async(any(feature = "surf_async"), async_std::test)
)]
async fn test_geo_index() {
    test_setup();
    let collection_name = "test_collection";
    let index_name = "idx_geo_test";
    let conn = connection().await;

    let mut database = conn.db("test_db").await.unwrap();

    let index = GeoIndex::builder()
        .name(index_name.to_string())
        .fields(vec!["password".to_string()])
        .build();

    let index = Index::Geo(index);

    let result = database
        .create_index(collection_name, &index)
        .await
        .unwrap();

    if let Index::Geo(index) = result {
        let id = index.id.as_ref().unwrap();
        let delete_result = database.delete_index(&id).await.unwrap();

        assert!(index.id.is_some());
        assert_eq!(index.name, Some(index_name.to_string()));
        assert_eq!(&delete_result.id, id);
    } else {
        panic!("Wrong index type");
    }
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
    let conn = connection().await;

    let mut database = conn.db("test_db").await.unwrap();

    let index = TtlIndex::builder()
        .name(index_name.to_string())
        .fields(vec!["password".to_string()])
        .build();

    let index = Index::Ttl(index);

    let result = database
        .create_index(collection_name, &index)
        .await
        .unwrap();

    if let Index::Ttl(index) = result {
        let id = index.id.as_ref().unwrap();
        let delete_result = database.delete_index(&id).await.unwrap();

        assert!(index.id.is_some());
        assert_eq!(index.name, Some(index_name.to_string()));
        assert_eq!(&delete_result.id, id);
    } else {
        panic!("Wrong index type");
    }
}

#[maybe_async::test(
    any(feature = "reqwest_blocking"),
    async(any(feature = "reqwest_async"), tokio::test),
    async(any(feature = "surf_async"), async_std::test)
)]
async fn test_fulltext_index() {
    test_setup();
    let collection_name = "test_collection";
    let index_name = "idx_full_test";
    let conn = connection().await;

    let mut database = conn.db("test_db").await.unwrap();

    let index = FulltextIndex::builder()
        .name(index_name.to_string())
        .fields(vec!["password".to_string()])
        .build();

    let index = Index::Fulltext(index);

    let result = database
        .create_index(collection_name, &index)
        .await
        .unwrap();

    if let Index::Fulltext(index) = result {
        let id = index.id.as_ref().unwrap();
        let delete_result = database.delete_index(&id).await.unwrap();

        assert!(index.id.is_some());
        assert_eq!(index.name, Some(index_name.to_string()));
        assert_eq!(&delete_result.id, id);
    } else {
        panic!("Wrong index type");
    }
}

#[maybe_async::test(
    any(feature = "reqwest_blocking"),
    async(any(feature = "reqwest_async"), tokio::test),
    async(any(feature = "surf_async"), async_std::test)
)]
async fn test_list_indexes() {
    test_setup();
    let collection_name = "test_collection";
    let conn = connection().await;

    let mut database = conn.db("test_db").await.unwrap();
    let list = database.indexes(collection_name).await.unwrap();

    assert!(list.indexes.len() > 0);
}
