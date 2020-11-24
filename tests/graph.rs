#![allow(unused_imports)]
#![allow(unused_parens)]

use log::trace;
use pretty_assertions::assert_eq;
use serde_json::{json, Value};

use crate::common::{collection, connection};
use arangors::client::ClientExt;
use arangors::{
    collection::{
        options::{ChecksumOptions, PropertiesOptions},
        response::Status,
        CollectionType,
    },
    graph::*,
    ClientError, Connection, Database, Document,
};
use common::{get_arangodb_host, get_normal_password, get_normal_user, test_setup};

pub mod common;

#[maybe_async::maybe_async]
async fn clean_graphs<C: ClientExt>(db: &Database<C>) {
    let count = db.graphs().await.unwrap();
    log::trace!("{} graphs found, deleting...", count.graphs.len());
    println!("{} graphs found, deleting...", count.graphs.len());
    for a in count.graphs.iter() {
        db.drop_graph(&a.name, false).await.unwrap();
    }
    let count = db.graphs().await.unwrap();
    log::trace!("{} graphs found after deletion", count.graphs.len());
    println!("{} graphs found after deletion", count.graphs.len());
}

#[maybe_async::test(
    any(feature = "reqwest_blocking"),
    async(any(feature = "reqwest_async"), tokio::test),
    async(any(feature = "surf_async"), async_std::test)
)]
async fn test_simple_graph() {
    test_setup();
    let conn = connection().await;

    let database = conn.db("test_db").await.unwrap();
    // Cleanup
    clean_graphs(&database).await;

    let graph = Graph::builder()
        .name("test_graph".to_string())
        .edge_definitions(vec![EdgeDefinition {
            collection: "some_edge".to_string(),
            from: vec!["from_collection".to_string()],
            to: vec!["to_collection".to_string()],
        }])
        .build();
    let result = database.create_graph(graph, true).await.unwrap();
    assert_eq!(result.name, "test_graph".to_string());
    assert!(result.is_disjoint.is_none());
    assert!(result.is_smart.is_none());
    assert!(result.orphan_collections.is_empty());
    assert!(result.options.is_none());
}

#[maybe_async::test(
    any(feature = "reqwest_blocking"),
    async(any(feature = "reqwest_async"), tokio::test),
    async(any(feature = "surf_async"), async_std::test)
)]
async fn test_graph_retrieval() {
    test_setup();
    let conn = connection().await;

    let database = conn.db("test_db").await.unwrap();
    // Cleanup
    clean_graphs(&database).await;

    let graph1 = Graph::builder()
        .name("test_graph1".to_string())
        .edge_definitions(vec![EdgeDefinition {
            collection: "some_edge1".to_string(),
            from: vec!["from_collection_1".to_string()],
            to: vec!["to_collection".to_string()],
        }])
        .build();
    let graph2 = Graph::builder()
        .name("test_graph2".to_string())
        .edge_definitions(vec![EdgeDefinition {
            collection: "some_edge2".to_string(),
            from: vec!["from_collection_2".to_string()],
            to: vec!["to_collection".to_string()],
        }])
        .build();
    let graph3 = Graph::builder()
        .name("test_graph3".to_string())
        .edge_definitions(vec![EdgeDefinition {
            collection: "some_edge3".to_string(),
            from: vec!["from_collection_3".to_string()],
            to: vec!["to_collection".to_string()],
        }])
        .build();
    database.create_graph(graph1, true).await.unwrap();
    database.create_graph(graph2, true).await.unwrap();
    database.create_graph(graph3, true).await.unwrap();

    let count = database.graphs().await.unwrap();
    log::trace!("received: {:?}", count);
    assert!(count.graphs.len() >= 3);

    let result = database.graph("test_graph2").await.unwrap();
    assert_eq!(result.name, "test_graph2");
}
