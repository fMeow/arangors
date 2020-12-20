//! This module facilitates the building of new named graphs as well as the retrieval
//! of existing indexes in ArangoDB.
//!
//! The various structures are following the HTTP specification as detailed in this ArangoDB
//! [section](https://www.arangodb.com/docs/stable/http/gharial-management.html)
//!
//! For detailed information about ArangoDB named graphs, please check out the official
//! ArangoDB [documentation](https://www.arangodb.com/docs/stable/http/gharial.html).
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

pub(crate) const GHARIAL_API_PATH: &str = "_api/gharial";

/// Represents a Named Graph in ArangoDB.
#[derive(Debug, Clone, Serialize, Deserialize, Default, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Graph {
    /// Name of the graph
    #[builder(default)]
    pub name: String,
    /// An array of definitions for the relations of the graph.
    #[builder(default)]
    pub edge_definitions: Vec<EdgeDefinition>,
    /// An array of additional vertex collections. Documents within these collections do not have edges within this graph.
    #[builder(default)]
    #[serde(skip_serializing_if = "Vec::is_empty", default = "Vec::new")]
    pub orphan_collections: Vec<String>,
    /// Define if the created graph should be smart (Enterprise Edition only).
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_smart: Option<bool>,
    /// Whether to create a Disjoint SmartGraph instead of a regular SmartGraph (Enterprise Edition only).
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_disjoint: Option<bool>,
    /// a JSON object to define options for creating collections within this graph.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<GraphOptions>,
}

/// Represents the available options for a [`Graph`] Creation
///
/// [`Graph`]: struct.Graph.html
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphOptions {
    /// Only has effect in Enterprise Edition and it is required if isSmart is true.
    /// The attribute name that is used to smartly shard the vertices of a graph.
    /// Every vertex in this SmartGraph has to have this attribute. Cannot be modified later.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_graph_attribute: Option<String>,
    /// The number of shards that is used for every collection within this graph. Cannot be modified later.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_shards: Option<u32>,
    /// The replication factor used when initially creating collections for this graph.
    /// Can be set to "satellite" to create a SatelliteGraph, which will ignore numberOfShards,
    /// minReplicationFactor and writeConcern (Enterprise Edition only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_factor: Option<u32>,
    /// Write concern for new collections in the graph.
    /// It determines how many copies of each shard are required to be in sync on the different DB-Servers.
    /// If there are less then these many copies in the cluster a shard will refuse to write.
    /// Writes to shards with enough up-to-date copies will succeed at the same time however.
    /// The value of writeConcern can not be larger than replicationFactor. (cluster only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_concern: Option<u32>,
}

/// Represents one Edge definition for a [`Graph`] Creation.
///
/// [`Graph`]: struct.Graph.html
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EdgeDefinition {
    /// Name of the edge collection
    pub collection: String,
    /// List of the `_from` collection names
    pub from: Vec<String>,
    /// List of the `_to` collection names
    pub to: Vec<String>,
}

/// Represents a collection of [`Graphs`] on a database in ArangoDB.
///
/// [`Graphs`]: struct.Graph.html
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphCollection {
    pub graphs: Vec<Graph>,
}

/// Represents a [`Graph`] as returned by ArangoDB after a HTTP retrieval
///
/// [`Graph`]: struct.Graph.html
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphResponse {
    pub graph: Graph,
}
