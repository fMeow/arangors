use serde::{
    de::{Deserializer, Error as DeError},
    Deserialize,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionInfo {
    pub count: Option<u32>,
    pub id: String,
    pub name: String,
    pub globally_unique_id: String,
    pub is_system: bool,
    pub status: CollectionStatus,
    pub r#type: CollectionType,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum CollectionStatus {
    NewBorn = 1,
    Unloaded = 2,
    Loaded = 3,
    Unloading = 4,
    Deleted = 5,
    Loading = 6,
}

impl<'de> Deserialize<'de> for CollectionStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u8::deserialize(deserializer)?;
        match value {
            1 => Ok(CollectionStatus::NewBorn),
            2 => Ok(CollectionStatus::Unloaded),
            3 => Ok(CollectionStatus::Loaded),
            4 => Ok(CollectionStatus::Unloading),
            5 => Ok(CollectionStatus::Deleted),
            6 => Ok(CollectionStatus::Loading),
            _ => Err(DeError::custom(
                "Undefined behavior. If the crate breaks after an upgrade of ArangoDB, please \
                 contact the author.",
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum CollectionType {
    Document = 2,
    Edge = 3,
}

impl<'de> Deserialize<'de> for CollectionType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u8::deserialize(deserializer)?;
        match value {
            2 => Ok(CollectionType::Document),
            3 => Ok(CollectionType::Edge),
            _ => Err(DeError::custom(
                "Undefined behavior. If the crate breaks after an upgrade of ArangoDB, please \
                 contact the author.",
            )),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionKeyOptions {
    pub allow_user_keys: bool,
    pub increment: Option<u32>,
    pub last_value: Option<u32>,
    pub offset: Option<u32>,
    pub r#type: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionProperties {
    #[serde(flatten)]
    pub info: CollectionInfo,
    #[serde(flatten)]
    pub detail: CollectionDetails,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionDetails {
    pub status_string: String,
    pub key_options: CollectionKeyOptions,
    pub wait_for_sync: bool,
    pub write_concern: u16,
    #[cfg(rocksdb)]
    pub cache_enabled: bool,
    #[cfg(rocksdb)]
    pub object_id: String,
    #[cfg(mmfiles)]
    pub is_volatile: bool,
    #[cfg(mmfiles)]
    pub do_compact: bool,
    #[cfg(mmfiles)]
    pub journal_size: usize,
    #[cfg(mmfiles)]
    pub index_buckets: usize,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArangoIndex {
    pub count: Option<u32>,
    pub size: Option<u32>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Figures {
    pub indexes: ArangoIndex,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionStatistics {
    /// The number of documents currently present in the collection.
    pub count: Option<u32>,
    /// metrics of the collection
    pub figures: Figures,

    #[serde(flatten)]
    pub info: CollectionInfo,
    #[serde(flatten)]
    pub detail: CollectionDetails,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionRevision {
    // pub uses_revisions_as_document_ids: Option<bool>,
    // pub sync_by_revision: bool,
    // pub min_revision: u32,
    // These 3 properties are for Arangodb 3.7
    pub revision: String,
    #[serde(flatten)]
    pub info: CollectionInfo,
    #[serde(flatten)]
    pub detail: CollectionDetails,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionChecksum {
    pub revision: String,
    pub checksum: String,
    #[serde(flatten)]
    pub info: CollectionInfo,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionPropertiesOptions {
    /// If true then creating or changing a document will wait until the data
    /// has been synchronized to disk.
    pub wait_for_sync: Option<bool>,
    // TODO need to implement this with feature gate between versions maybe
    //  for ArangoDB 3.7
    // pub schema: Option<SchemaRules>,
}
