//! Types of response related to collection
use crate::collection::{options::KeyOptions, CollectionType};
use serde::{
    de::{Deserializer, Error as DeError},
    Deserialize,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    pub count: Option<u32>,
    pub id: String,
    pub name: String,
    pub globally_unique_id: String,
    pub is_system: bool,
    pub status: Status,
    #[serde(rename = "type")]
    pub collection_type: CollectionType,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Status {
    NewBorn = 1,
    Unloaded = 2,
    Loaded = 3,
    Unloading = 4,
    Deleted = 5,
    Loading = 6,
}

impl<'de> Deserialize<'de> for Status {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u8::deserialize(deserializer)?;
        match value {
            1 => Ok(Status::NewBorn),
            2 => Ok(Status::Unloaded),
            3 => Ok(Status::Loaded),
            4 => Ok(Status::Unloading),
            5 => Ok(Status::Deleted),
            6 => Ok(Status::Loading),
            _ => Err(DeError::custom(
                "Undefined behavior. If the crate breaks after an upgrade of ArangoDB, please \
                 contact the author.",
            )),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties {
    #[serde(flatten)]
    pub info: Info,
    #[serde(flatten)]
    pub detail: Details,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Details {
    pub status_string: String,
    pub key_options: KeyOptions,
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
pub struct Statistics {
    /// The number of documents currently present in the collection.
    pub count: Option<u32>,
    /// metrics of the collection
    pub figures: Figures,

    #[serde(flatten)]
    pub info: Info,
    #[serde(flatten)]
    pub detail: Details,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Revision {
    // pub uses_revisions_as_document_ids: Option<bool>,
    // pub sync_by_revision: bool,
    // pub min_revision: u32,
    // These 3 properties are for Arangodb 3.7
    pub revision: String,
    #[serde(flatten)]
    pub info: Info,
    #[serde(flatten)]
    pub detail: Details,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Checksum {
    pub revision: String,
    pub checksum: String,
    #[serde(flatten)]
    pub info: Info,
}
