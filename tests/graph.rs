#![allow(unused_imports)]
#![allow(unused_parens)]

use log::trace;
use pretty_assertions::assert_eq;
use serde_json::{json, Value};

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

use crate::common::{collection, connection};

pub mod common;

#[maybe_async::maybe_async]
async fn drop_all_graphs(db: &Database, names: Vec<&str>) {
    for name in names.iter() {
        drop_graph(db, name).await;
    }
}

#[maybe_async::maybe_async]
async fn drop_graph(db: &Database, name: &str) {
    match db.drop_graph(name, false).await {
        Ok(()) => (),
        Err(err) => println!("Failed to drop graph: {:?}", err),
    }
}

#[maybe_async::test(feature = "blocking", async(not(feature = "blocking"), tokio::test))]
async fn test_simple_graph() {
    test_setup();
    let conn = connection().await;

    let database = conn.db("test_db").await.unwrap();
    // Cleanup
    drop_graph(&database, "test_graph").await;

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

#[maybe_async::test(feature = "blocking", async(not(feature = "blocking"), tokio::test))]
async fn test_complex_graph() {
    test_setup();
    let conn = connection().await;

    let database = conn.db("test_db").await.unwrap();
    // Cleanup
    drop_graph(&database, "test_complex_graph").await;

    let graph = Graph::builder()
        .name("test_complex_graph".to_string())
        .edge_definitions(vec![EdgeDefinition {
            collection: "some_edge".to_string(),
            from: vec!["from_collection".to_string()],
            to: vec!["to_collection".to_string()],
        }])
        .orphan_collections(vec!["some_collection".to_string()])
        .is_smart(Some(true))
        .is_disjoint(Some(false))
        .options(Some(GraphOptions {
            smart_graph_attribute: Some("region".to_string()),
            number_of_shards: Some(2),
            replication_factor: Some(10),
            write_concern: Some(8),
        }))
        .build();
    let result = database.create_graph(graph, true).await.unwrap();
    assert_eq!(result.name, "test_complex_graph".to_string());
    assert_eq!(result.orphan_collections.len(), 1);
    // Would work only with Enterprise Edition
    //
    // assert_eq!(result.is_disjoint.unwrap(), false);
    // assert_eq!(result.is_smart.unwrap(), true);
    // assert!(result.options.is_some());
    // let options = result.options.unwrap();
    // assert_eq!(options.number_of_shards.unwrap(), 2);
    // assert_eq!(options.replication_factor.unwrap(),10);
    // assert_eq!(options.write_concern.unwrap(), 8);
    // assert_eq!(options.smart_graph_attribute.unwrap(), "region".to_string());
}

#[maybe_async::test(feature = "blocking", async(not(feature = "blocking"), tokio::test))]
async fn test_graph_retrieval() {
    test_setup();
    let conn = connection().await;

    let database = conn.db("test_db").await.unwrap();
    // Cleanup
    drop_all_graphs(&database, vec!["test_graph1", "test_graph2", "test_graph3"]).await;

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

// This tests the default value of `orphanCollections` which can't be optional but can be empty
#[test]
fn minimal_serialization_works() {
    let json = json!(
     {
         "name": "GraphName",
         "edgeDefinitions": []
     }
    );
    let graph: Graph = serde_json::from_value(json).unwrap();
    assert!(graph.orphan_collections.is_empty());
}
