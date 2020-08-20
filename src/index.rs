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
//! An index of type [`Primary`] cannot be created and is only available for
//! the retrieval of existing indexes, as ArangoDB creates a primary index on every
//! collection.
//! For detailed information about ArangoDB indexes, please check out the official
//! ArangoDB [documentation](https://www.arangodb.com/docs/stable/http/indexes.html).
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

/// Represents an [`Index`] in ArangoDB. The following types are
/// supported:
/// * Fulltext
/// * Geo
/// * Hash
/// * Persistent
/// * Skiplist
/// * Ttl (Time to live)
///
/// As different settings may be applied to different index types, use the [`settings`] field
/// on the index to specify the exact `type` of the index including the required settings.
///
/// # Example
/// ```ignore
///     let database = conn.db("test_db").await.unwrap();
///
///     let index = Index::builder()
///         .name(index_name)
///         .fields(vec!["password".to_string()])
///         .settings(IndexSettings::Persistent {
///             unique: true,
///             sparse: false,
///             deduplicate: false,
///         })
///         .build();
///
///     let index = database
///         .create_index(collection_name, &index)
///         .await?;
/// ```
#[derive(Debug, Serialize, Deserialize, Default, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Index {
    #[builder(default)]
    pub fields: Vec<String>,
    #[builder(default, setter(into))]
    pub name: String,
    #[builder(default)]
    pub id: String,
    #[builder(default)]
    pub is_newly_created: Option<bool>,
    #[builder(default)]
    pub selectivity_estimate: Option<u8>,
    #[builder(default)]
    pub in_background: Option<bool>,
    #[serde(flatten)]
    #[builder(default)]
    pub settings: IndexSettings,
}

/// Settings for the different index types. This `enum` also sets the index type.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum IndexSettings {
    Primary {
        unique: bool,
        sparse: bool,
    },
    Persistent {
        unique: bool,
        sparse: bool,
        deduplicate: bool,
    },
    Hash {
        unique: bool,
        sparse: bool,
        deduplicate: bool,
    },
    Skiplist {
        unique: bool,
        sparse: bool,
        deduplicate: bool,
    },
    #[serde(rename_all = "camelCase")]
    Ttl {
        expire_after: u32,
    },
    #[serde(rename_all = "camelCase")]
    Geo {
        geo_json: bool,
    },
    #[serde(rename_all = "camelCase")]
    Fulltext {
        min_length: u32,
    },
}

impl Default for IndexSettings {
    fn default() -> Self {
        IndexSettings::Persistent {
            unique: false,
            sparse: false,
            deduplicate: false,
        }
    }
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
