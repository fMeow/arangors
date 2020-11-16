//! Types of response related to collection
use serde::{Deserialize, Serialize, Serializer};
use typed_builder::TypedBuilder;

use crate::collection::CollectionType;

/// Options for create a collection
#[derive(Serialize, PartialEq, TypedBuilder)]
#[builder(doc)]
#[serde(rename_all = "camelCase")]
pub struct CreateParameters {
    /// Default is 1 which means the server will only report success back to the
    /// client if all replicas have created the collection. Set to 0 if you want
    /// faster server responses and don’t care about full replication.
    #[serde(skip_serializing_if = "Option::is_none", serialize_with = "bool2int")]
    #[builder(default, setter(strip_option))]
    wait_for_sync_replication: Option<bool>,
    /// Default is 1 which means the server will check if there are enough
    /// replicas available at creation time and bail out otherwise. Set to 0 to
    /// disable this extra check.
    #[serde(skip_serializing_if = "Option::is_none", serialize_with = "bool2int")]
    #[builder(default, setter(strip_option))]
    enforce_replication_factor: Option<bool>,
}
impl Default for CreateParameters {
    fn default() -> Self {
        Self::builder().build()
    }
}

fn bool2int<S>(v: &Option<bool>, ser: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if v.is_none() || *v.as_ref().unwrap() {
        ser.serialize_i8(1)
    } else {
        ser.serialize_i8(0)
    }
}
/// Options for create a collection
#[derive(Serialize, PartialEq, TypedBuilder)]
#[builder(doc)]
#[serde(rename_all = "camelCase")]
pub struct CreateOptions<'a> {
    name: &'a str,

    /// the type of the collection to create
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    collection_type: Option<CollectionType>,

    /// If true then the data is synchronized to disk before returning from a
    /// document create, update, replace or removal operation. (default: false)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    wait_for_sync: Option<bool>,

    /// isSystem: If true, create a system collection. In this case
    /// collection-name should start with an underscore. End users should
    /// normally create non-system collections only. API implementors may be
    /// required to create system collections in very special occasions, but
    /// normally a regular collection will do. (The default is false)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    is_system: Option<bool>,

    /// additional options for key generation
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    key_options: Option<KeyOptions>,

    /// Optional object that specifies the collection level schema for
    /// documents. The attribute keys rule, level and message must follow the
    /// rules documented in Document Schema Validation https://www.arangodb.com/docs/devel/document-schema-validation.html
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    schema: Option<serde_json::Value>,

    /// This attribute specifies the name of the sharding strategy to use for
    /// the collection. Since ArangoDB 3.4 there are different sharding
    /// strategies to select from when creating a new collection. The selected
    /// shardingStrategy value will remain fixed for the collection and cannot
    /// be changed afterwards. This is important to make the collection keep its
    /// sharding settings and always find documents already distributed to
    /// shards using the same initial sharding algorithm. The available
    /// sharding strategies are:
    ///
    /// - community-compat: default sharding used by ArangoDB Community Edition
    ///   before version 3.4
    /// - enterprise-compat: default sharding used by ArangoDB Enterprise
    ///   Edition before version 3.4
    /// - enterprise-smart-edge-compat: default sharding used by smart edge
    ///   collections in ArangoDB Enterprise Edition before version 3.4
    /// - hash: default sharding used for new collections starting from version
    ///   3.4 (excluding smart edge collections)
    /// - enterprise-hash-smart-edge: default sharding used for new smart edge
    ///   collections starting from version 3.4
    ///
    /// If no sharding strategy is specified, the default will be hash for all
    /// collections, and enterprise-hash-smart-edge for all smart edge
    /// collections (requires the Enterprise Edition of ArangoDB). Manually
    /// overriding the sharding strategy does not yet provide a benefit, but it
    /// may later in case other sharding strategies are added.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    sharding_strategy: Option<String>,

    /// whether or not the collection will be compacted (default is true) This
    /// option is meaningful for the MMFiles storage engine only.
    #[cfg(feature = "mmfiles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    do_compat: Option<bool>,

    /// The maximal size of a journal or datafile in bytes. The value must be at
    /// least 1048576 (1 MiB). (The default is a configuration parameter) This
    /// option is meaningful for the MMFiles storage engine only.
    #[cfg(feature = "mmfiles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    journal_size: Option<usize>,
    /// If true then the collection data is kept in-memory only and not made
    /// persistent. Unloading the collection will cause the collection data to
    /// be discarded. Stopping or re-starting the server will also cause full
    /// loss of data in the collection. Setting this option will make the
    /// resulting collection be slightly faster than regular collections because
    /// ArangoDB does not enforce any synchronization to disk and does not
    /// calculate any CRC checksums for datafiles (as there are no datafiles).
    /// This option should therefore be used for cache-type collections only,
    /// and not for data that cannot be re-created otherwise. (The default is
    /// false) This option is meaningful for the MMFiles storage engine only.
    #[cfg(feature = "mmfiles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    is_volatile: Option<bool>,

    /// (The default is 1): in a cluster, this value determines the number of
    /// shards to create for the collection. In a single server setup, this
    /// option is meaningless.
    #[cfg(feature = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    number_of_shards: Option<usize>,
    /// (The default is [ “_key” ]): in a cluster, this attribute determines
    /// which document attributes are used to determine the target shard for
    /// documents. Documents are sent to shards based on the values of their
    /// shard key attributes. The values of all shard key attributes in a
    /// document are hashed, and the hash value is used to determine the target
    /// shard. Note: Values of shard key attributes cannot be changed once set.
    /// This option is meaningless in a single server setup.
    #[cfg(feature = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    shard_keys: Option<Vec<String>>,

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
    #[cfg(feature = "cluster")]
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
    #[cfg(feature = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    write_concern: Option<usize>,

    /// (The default is ”“): in an Enterprise Edition cluster, this attribute
    /// binds the specifics of sharding for the newly created collection to
    /// follow that of a specified existing collection. Note: Using this
    /// parameter has consequences for the prototype collection. It can no
    /// longer be dropped, before the sharding-imitating collections are
    /// dropped. Equally, backups and restores of imitating collections alone
    /// will generate warnings (which can be overridden) about missing sharding
    /// prototype.
    #[cfg(feature = "enterprise")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    distribute_shards_like: Option<String>,

    /// In an Enterprise Edition cluster, this attribute determines an attribute
    /// of the collection that must contain the shard key value of the
    /// referred-to SmartJoin collection. Additionally, the shard key for a
    /// document in this collection must contain the value of this attribute,
    /// followed by a colon, followed by the actual primary key of the document.
    ///
    /// This feature can only be used in the Enterprise Edition and requires the
    /// distributeShardsLike attribute of the collection to be set to the name
    /// of another collection. It also requires the shardKeys attribute of the
    /// collection to be set to a single shard key attribute, with an additional
    /// ‘:’ at the end. A further restriction is that whenever documents are
    /// stored or updated in the collection, the value stored in the
    /// smartJoinAttribute must be a string.
    #[cfg(feature = "enterprise")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    smart_join_attribute: Option<String>,
}

fn is_true(x: &bool) -> bool {
    *x
}

#[derive(Debug, Deserialize, Serialize, TypedBuilder, PartialEq)]
#[builder(doc)]
#[serde(rename_all = "camelCase")]
pub struct KeyOptions {
    /// if set to true, then it is allowed to supply own key values in the _key
    /// attribute of a document. If set to false, then the key generator will
    /// solely be responsible for generating keys and supplying own key values
    /// in the _key attribute of documents is considered an error.
    #[serde(skip_serializing_if = "is_true")]
    #[builder(default = true)]
    pub allow_user_keys: bool,

    /// specifies the type of the key generator. The currently available
    /// generators are traditional and autoincrement.
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    #[builder(default, setter(strip_option))]
    pub key_type: Option<String>,

    /// increment value for autoincrement key generator. Not used for other key
    /// generator types.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub increment: Option<u32>,

    /// Initial offset value for autoincrement key generator. Not used for other
    /// key generator types.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub offset: Option<u32>,

    #[serde(skip_serializing)]
    #[builder(setter(skip), default = None)]
    pub last_value: Option<u32>,
}

impl Default for KeyOptions {
    fn default() -> Self {
        Self::builder().build()
    }
}

/// Options for checksum
#[derive(Serialize, Deserialize, PartialEq, TypedBuilder)]
#[builder(doc)]
#[serde(rename_all = "camelCase")]
pub struct ChecksumOptions {
    /// By setting the optional query parameter withRevisions to true, then
    /// revision ids (_rev system attributes) are included in the
    /// checksumming.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    with_revision: Option<bool>,
    /// By providing the optional query parameter withData with a value of true,
    /// the user-defined document attributes will be included in the
    /// calculation too.
    ///
    /// Note: Including user-defined attributes will make
    /// the checksumming slower.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    with_data: Option<bool>,
}

impl Default for ChecksumOptions {
    fn default() -> Self {
        Self::builder().build()
    }
}

#[derive(Debug, Deserialize, Serialize, TypedBuilder)]
#[builder(doc)]
#[serde(rename_all = "camelCase")]
pub struct PropertiesOptions {
    /// If true then creating or changing a document will wait until the data
    /// has been synchronized to disk.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    wait_for_sync: Option<bool>,
    /* TODO need to implement this with feature gate between versions maybe
     *  for ArangoDB 3.7
     * schema: Option<SchemaRules>, */
}

impl Default for PropertiesOptions {
    fn default() -> Self {
        Self::builder().build()
    }
}
