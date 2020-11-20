use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[cfg(feature = "cluster")]
use std::collections::HashMap;

/// Options for create a collection
#[derive(Serialize, PartialEq, TypedBuilder)]
#[builder(doc)]
#[serde(rename_all = "camelCase")]
#[cfg(feature = "cluster")]
pub struct CreateDatabaseOptions {
    /// The sharding method to use for new collections in this database.
    /// Valid values are: “”, “flexible”, or “single”. The first two are equivalent
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    sharding: Option<String>,

    /// (The default is 1): in a cluster, this attribute determines how many
    /// copies of each shard are kept on different DB-Servers. The value 1 means
    /// that only one copy (no synchronous replication) is kept. A value of k
    /// means that k-1 replicas are kept. It can also be the string "satellite"
    /// for a SatelliteCollection, where the replication factor is matched to
    /// the number of DB-Servers.
    ///
    /// Any two copies reside on different DB-Servers. Replication between them
    /// is synchronous, that is, every write operation to the “leader” copy will
    /// be replicated to all “follower” replicas, before the write operation is
    /// reported successful.
    ///
    /// If a server fails, this is detected automatically and one of the servers
    /// holding copies take over, usually without an error being reported.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    replication_factor: Option<usize>,

    /// Write concern for this collection (default: 1).
    ///
    /// It determines how many copies of each shard are required to be in sync
    /// on the different DB-Servers. If there are less then these many copies in
    /// the cluster a shard will refuse to write. Writes to shards with enough
    /// up-to-date copies will succeed at the same time however. The value of
    /// writeConcern can not be larger than replicationFactor. (cluster only)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    write_concern: Option<usize>,
}

#[derive(Serialize, PartialEq, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CreateDatabase<'a> {
    name: &'a str,

    #[cfg(feature = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    options: Option<CreateDatabaseOptions>,
}

#[derive(Serialize, PartialEq, Deserialize)]
pub enum ClusterRole {
    Coordinator,
    DBServer,
    Agent,
}

#[derive(Serialize, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Engine {
    RocksDB,
    MMFiles,
}

#[derive(Serialize, PartialEq, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ClusterStatus {
    Good,
    Bad,
    Failed,
}

#[derive(Serialize, PartialEq, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum SyncStatus {
    Serving,
    Startup,
    Stopping,
    Stopped,
    Shutdown,
    Undefined,
    Unknown,
}

#[derive(Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
#[cfg(feature = "cluster")]
pub struct ServerHealth {
    pub endpoint: String,

    pub role: ClusterRole,

    pub status: ClusterStatus,

    pub engine: Engine,

    pub version: String,

    pub leader: Option<String>,

    pub sync_status: Option<SyncStatus>,
}

#[derive(Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
#[cfg(feature = "cluster")]
pub struct ClusterHealth {
    pub cluster_id: String,

    pub health: HashMap<String, ServerHealth>,
}
