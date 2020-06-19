use std::sync::Arc;

use maybe_async::maybe_async;
use serde::{
    de::{Deserializer, Error as DeError},
    Deserialize,
};
use url::Url;

use crate::client::ClientExt;

use super::{Database, Document};
use crate::response::serialize_response;
use crate::ClientError;

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
pub struct CollectionDetails {
    pub cache_enabled: bool,
    pub count: Option<u32>,
    pub globally_unique_id: String,
    pub id: String,
    pub is_system: bool,
    pub key_options: CollectionKeyOptions,
    pub name: String,
    pub object_id: String,
    pub status: u16,
    pub status_string: String,
    pub r#type: u16,
    pub wait_for_sync: bool,
    pub write_concern: u16,
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
    pub cache_enabled: bool,
    pub globally_unique_id: String,
    pub id: String,
    pub is_system: bool,
    pub key_options: CollectionKeyOptions,
    pub name: String,
    pub object_id: String,
    pub status: u16,
    pub status_string: String,
    pub r#type: u16,
    pub wait_for_sync: bool,
    pub write_concern: u16,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionRevision {
    // pub uses_revisions_as_document_ids: Option<bool>,
    // pub sync_by_revision: bool,
    // pub min_revision: u32,
    /// These 3 properties are for Arangodb 3.7
    pub revision: String,
    pub cache_enabled: bool,
    pub globally_unique_id: String,
    pub id: String,
    pub is_system: bool,
    pub key_options: CollectionKeyOptions,
    pub name: String,
    pub object_id: String,
    pub status: u16,
    pub status_string: String,
    pub r#type: u16,
    pub wait_for_sync: bool,
    pub write_concern: u16,
}

#[derive(Debug, Clone)]
pub struct Collection<'a, C: ClientExt> {
    id: String,
    name: String,
    collection_type: CollectionType,
    ///
    /// https://www.arangodb.com/docs/devel/http/collection-getting.html
    ///  Weirdly the good path for a collection is different from what the official doc says.
    base_url: Url,
    session: Arc<C>,
    phantom: &'a (),
}

impl<'a, C: ClientExt> Collection<'a, C> {
    /// Construct Collection given
    ///  Base url should be like `http://localhost:8529/_db/mydb/_api/collection/{collection-name}`
    pub(crate) fn new<T: Into<String>>(
        database: &'a Database<C>,
        name: T,
        id: T,
        collection_type: CollectionType,
    ) -> Collection<'a, C> {
        let name = name.into();
        let path = format!("_api/collection/{}/", name.as_str());
        let url = database.get_url().join(path.as_str()).unwrap();
        Collection {
            name: name,
            id: id.into(),
            session: database.get_session(),
            base_url: url,
            collection_type,
            phantom: database.phantom,
        }
    }

    pub(crate) fn from_response(
        database: &'a Database<C>,
        collection: &CollectionResponse,
    ) -> Collection<'a, C> {
        Self::new(
            database,
            collection.name.to_owned(),
            collection.id.to_owned(),
            collection.r#type.clone(),
        )
    }

    pub fn collection_type(&self) -> &CollectionType {
        &self.collection_type
    }

    pub fn id(&self) -> &str {
        self.id.as_str()
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn url(&self) -> &Url {
        &self.base_url
    }

    pub fn session(&self) -> Arc<C> {
        Arc::clone(&self.session)
    }

    /// Truncate current collection.
    ///
    /// # Note
    /// this function would make a request to arango server.
    pub fn truncate(&self) {
        unimplemented!()
    }

    /// Fetch the properties of collection
    ///
    /// # Note
    /// this function would make a request to arango server.
    #[maybe_async]
    pub async fn properties(&self) -> Result<CollectionDetails, ClientError> {
        let url = self.base_url.join(&format!("properties")).unwrap();
        let resp: CollectionDetails = serialize_response(self.session.get(url, "").await?.text())?;
        Ok(resp)
    }

    /// Counts the documents in this collection
    ///
    /// # Note
    /// this function would make a request to arango server.
    #[maybe_async]
    pub async fn document_count(&self) -> Result<CollectionDetails, ClientError> {
        let url = self.base_url.join(&format!("count")).unwrap();
        let resp: CollectionDetails = serialize_response(self.session.get(url, "").await?.text())?;
        Ok(resp)
    }
    /// Fetch the statistics of a collection
    ///
    /// the result also contains the number of documents and additional
    /// statistical information about the collection. **Note**: This will
    /// always load the collection into memory.
    ///
    /// **Note**: collection data that are stored in the write-ahead log only
    /// are not reported in the results. When the write-ahead log is
    /// collected, documents might be added to journals and datafiles of
    /// the collection, which may modify the figures of the collection.
    ///
    /// Additionally, the filesizes of collection and index parameter JSON
    /// files are not reported. These files should normally have a size of a
    /// few bytes each. Please also note that the fileSize values are reported
    /// in bytes and reflect the logical file sizes. Some filesystems may use
    /// optimisations (e.g. sparse files) so that the actual physical file size
    /// is somewhat different. Directories and sub-directories may also require
    /// space in the file system, but this space is not reported in the
    /// fileSize results.
    ///
    /// That means that the figures reported do not reflect the actual disk
    /// usage of the collection with 100% accuracy. The actual disk usage of a
    /// collection is normally slightly higher than the sum of the reported
    /// fileSize values. Still the sum of the fileSize values can still be used
    /// as a lower bound approximation of the disk usage.
    ///
    /// # Note
    /// this function would make a request to arango server.
    #[maybe_async]
    pub async fn statistics(&self) -> Result<CollectionStatistics, ClientError> {
        let url = self.base_url.join(&format!("figures")).unwrap();
        let resp: CollectionStatistics =
            serialize_response(self.session.get(url, "").await?.text())?;
        Ok(resp)
    }

    /// Retrieve the collections revision id
    ///
    /// The revision id is a server-generated string that clients can use to
    /// check whether data in a collection has changed since the last revision
    /// check.
    ///
    /// # Note
    /// this function would make a request to arango server.
    #[maybe_async]
    pub async fn revision_id(&self) -> Result<CollectionRevision, ClientError> {
        let url = self.base_url.join(&format!("revision")).unwrap();
        let resp: CollectionRevision = serialize_response(self.session.get(url, "").await?.text())?;
        Ok(resp)
    }
    /// Fetch a checksum for the specified collection
    ///
    /// Will calculate a checksum of the meta-data (keys and optionally
    /// revision ids) and optionally the document data in the collection.
    // The checksum can be used to compare if two collections on different ArangoDB
    // instances contain the same contents. The current revision of the collection
    // is returned too so one can make sure the checksums are calculated for the
    // same state of data.
    //
    // By default, the checksum will only be calculated on the _key system
    // attribute of the documents contained in the collection. For edge
    // collections, the system attributes _from and _to will also be included in
    // the calculation.
    //
    // By setting the optional query parameter withRevisions to true, then revision
    // ids (_rev system attributes) are included in the checksumming.
    //
    // By providing the optional query parameter withData with a value of true, the
    // user-defined document attributes will be included in the calculation too.
    // Note: Including user-defined attributes will make the checksumming slower.
    ///
    /// # Note
    /// this function would make a request to arango server.
    #[maybe_async]
    pub async fn checksum(&self) {
        unimplemented!()
    }

    /// Loads a collection into memory.
    ///
    /// # Note
    /// this function would make a request to arango server.
    #[maybe_async]
    pub async fn load(&self) {
        unimplemented!()
    }

    /// Removes a collection from memory. This call does not delete any
    /// documents. You can use the collection afterwards; in which case it will
    /// be loaded into memory, again.
    ///
    /// # Note
    /// this function would make a request to arango server.
    #[maybe_async]
    pub async fn unload(&self) {
        unimplemented!()
    }

    /// Load Indexes into Memory
    ///
    /// This route tries to cache all index entries of this collection into the
    /// main memory. Therefore it iterates over all indexes of the collection
    /// and stores the indexed values, not the entire document data, in memory.
    /// All lookups that could be found in the cache are much faster than
    /// lookups not stored in the cache so you get a nice performance boost. It
    /// is also guaranteed that the cache is consistent with the stored data.
    ///
    /// For the time being this function is only useful on RocksDB storage
    /// engine, as in MMFiles engine all indexes are in memory anyways.
    ///
    /// On RocksDB this function honors all memory limits, if the indexes you
    /// want to load are smaller than your memory limit this function
    /// guarantees that most index values are cached. If the index is larger
    /// than your memory limit this function will fill up values up to this
    /// limit and for the time being there is no way to control which indexes
    /// of the collection should have priority over others.
    ///
    /// On sucess this function returns an object with attribute result set to
    /// true
    ///
    /// # Note
    /// this function would make a request to arango server.
    #[maybe_async]
    pub async fn load_indexes(&self) {
        unimplemented!()
    }

    /// Changes the properties of a collection.
    ///
    /// # Note
    /// this function would make a request to arango server.
    #[maybe_async]
    pub async fn change_properties(&self) {
        unimplemented!()
    }

    /// Renames the collection
    ///
    /// # Note
    /// this function would make a request to arango server.
    #[maybe_async]
    pub async fn rename(&self) {
        unimplemented!()
    }

    /// Rotates the journal of a collection.
    ///
    /// The current journal of the collection will be closed and made a
    /// read-only datafile. The purpose of the rotate method is to make the
    /// data in the file available for compaction (compaction is only performed
    /// for read-only datafiles, and not for journals).
    ///
    /// Saving new data in the collection subsequently will create a new
    /// journal file automatically if there is no current journal.
    ///
    /// # Note
    /// this function would make a request to arango server.
    #[maybe_async]
    pub async fn rotate_journal(&self) {
        unimplemented!()
    }

    /// Creates a new document from the document given in the body, unless
    /// there is already a document with the _key given. If no _key is given, a
    /// new unique _key is generated automatically.
    ///
    /// # Note
    /// this function would make a request to arango server.
    #[maybe_async]
    pub async fn create_document<T>(&self, _doc: Document<T>) {
        unimplemented!()
    }

    /// Partially updates the document
    ///
    /// # Note
    /// this function would make a request to arango server.
    #[maybe_async]
    pub async fn update_document<T>(&self, _doc: Document<T>) {
        unimplemented!()
    }

    /// Replaces the document
    ///
    /// # Note
    /// this function would make a request to arango server.
    #[maybe_async]
    pub async fn replace_document<T>(&self, _doc: Document<T>) {
        unimplemented!()
    }

    /// Removes a document
    ///
    /// # Note
    /// this function would make a request to arango server.
    #[maybe_async]
    pub async fn remove_document<T>(&self, _doc: Document<T>) {
        unimplemented!()
    }
}

#[derive(Debug, Deserialize)]
pub struct CollectionResponse {
    pub id: String,
    pub name: String,
    pub status: CollectionStatus,
    pub r#type: CollectionType,
    #[serde(rename = "isSystem")]
    pub is_system: bool,
    #[serde(rename = "globallyUniqueId")]
    pub global_unique_id: String,
}

#[derive(Debug)]
pub enum CollectionStatus {
    NewBorn,
    Unloaded,
    Loaded,
    BeingUnload,
    Deleted,
    Loading,
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
            4 => Ok(CollectionStatus::BeingUnload),
            5 => Ok(CollectionStatus::Deleted),
            6 => Ok(CollectionStatus::Loading),
            _ => Err(DeError::custom(
                "Undefined behavior. If the crate breaks after an upgrade of ArangoDB, please \
                 contact the author.",
            )),
        }
    }
}

#[derive(Debug, Clone)]
pub enum CollectionType {
    Document,
    Edge,
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
