//! This module facilitates the building of new indexes as well as the retrieval
//! of existing indexes in ArangoDB.
//! The following types are supported:
//!
//! * Fulltext
//! * Geo
//! * Hash
//! * Persistent
//! * Skiplist
//! * Ttl (Time to live)
//!
//! To create a new `persistent`, `hash` or `skiplist` index, use the [`BasicIndex`] struct
//! wrapped inside the correct [`Index`] enum.
//! [`GeoIndex`], [`TtlIndex`] and [`FulltextIndex`] are represented by dedicated structs.
//!
//! An index of type [`Primary`] cannot be created and is only available for
//! the retrieval of existing indexes, as ArangoDB creates a primary index on every
//! collection.
//! For detailed information about ArangoDB indexes, please check out the official
//! ArangoDB [documentation](https://www.arangodb.com/docs/stable/http/indexes.html).
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

/// Represents the type of an [`Index`] in ArangoDB. The following types are
/// supported:
/// * Fulltext
/// * Geo
/// * Hash
/// * Persistent
/// * Skiplist
/// * Ttl (Time to live)
///
/// An index of type [`Primary`] cannot be created and is only available for
/// the retrieval of existing indexes, as ArangoDB creates a primary index on every
/// collection.
/// For detailed information about ArangoDB indexes, please check out the official
/// ArangoDB [documentation](https://www.arangodb.com/docs/stable/http/indexes.html).
///
/// # Example
/// ```ignore
///     let mut database = conn.db("test_db").await.unwrap();
///
///     let index = BasicIndex::builder()
///         .name(index_name.to_string())
///         .fields(vec!["password".to_string()])
///         .unique(true)
///         .build();
///
///     let index = Index::Persistent(index);
///
///     let result = database
///         .create_index(collection_name, &index)
///         .await
///         .unwrap();
/// ```
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum Index {
    Primary(BasicIndex),
    Persistent(BasicIndex),
    Hash(BasicIndex),
    Skiplist(BasicIndex),
    Geo(GeoIndex),
    Ttl(TtlIndex),
    Fulltext(FulltextIndex),
}

/// [`BaseIndex`] represents the base for:
///
/// * persistent
/// * hash
/// * skiplist
/// indexes.
#[derive(Debug, Serialize, Deserialize, Default, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct BasicIndex {
    pub fields: Vec<String>,
    #[builder(default, setter(into))]
    pub name: Option<String>,
    #[builder(default)]
    pub unique: bool,
    #[builder(default)]
    pub sparse: bool,
    #[builder(default = Some(false))]
    pub deduplicate: Option<bool>,

    #[builder(default)]
    pub id: Option<String>,
    #[builder(default)]
    pub is_newly_created: Option<bool>,
    #[builder(default)]
    pub selectivity_estimate: Option<u8>,
    #[builder(default)]
    pub error: Option<bool>,
    #[builder(default)]
    pub code: Option<u16>,
    #[builder(default)]
    pub in_background: Option<bool>,
}

/// Represents a persistent index on a collection in ArangoDB.
#[derive(Debug, Serialize, Deserialize, Default, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GeoIndex {
    pub fields: Vec<String>,
    #[builder(default, setter(into))]
    pub name: Option<String>,
    #[builder(default)]
    pub geo_json: bool,
    #[builder(default)]
    pub id: Option<String>,
    #[builder(default)]
    pub is_newly_created: Option<bool>,
    #[builder(default)]
    pub selectivity_estimate: Option<u8>,
    #[builder(default)]
    pub error: Option<bool>,
    #[builder(default)]
    pub code: Option<u16>,
    #[builder(default)]
    pub in_background: Option<bool>,
}

/// Represents a persistent index on a collection in ArangoDB.
#[derive(Debug, Serialize, Deserialize, Default, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct TtlIndex {
    pub fields: Vec<String>,
    #[builder(default, setter(into))]
    pub name: Option<String>,
    #[builder(default)]
    pub expire_after: u32,
    #[builder(default)]
    pub id: Option<String>,
    #[builder(default)]
    pub is_newly_created: Option<bool>,
    #[builder(default)]
    pub selectivity_estimate: Option<u8>,
    #[builder(default)]
    pub error: Option<bool>,
    #[builder(default)]
    pub code: Option<u16>,
    #[builder(default)]
    pub in_background: Option<bool>,
}

/// Represents a persistent index on a collection in ArangoDB.
#[derive(Debug, Serialize, Deserialize, Default, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct FulltextIndex {
    pub fields: Vec<String>,
    #[builder(default, setter(into))]
    pub name: Option<String>,
    #[builder(default)]
    pub min_length: u32,
    #[builder(default)]
    pub id: Option<String>,
    #[builder(default)]
    pub is_newly_created: Option<bool>,
    #[builder(default)]
    pub selectivity_estimate: Option<u8>,
    #[builder(default)]
    pub error: Option<bool>,
    #[builder(default)]
    pub code: Option<u16>,
    #[builder(default)]
    pub in_background: Option<bool>,
}

/// Represents a collection of indexes on a collection in ArangoDB.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexCollection {
    pub error: bool,
    pub code: u16,
    pub indexes: Vec<Index>,
}

/// Response from ArangoDB when deleting an index
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteIndexResponse {
    pub id: String,
    pub error: bool,
    pub code: u16,
}
