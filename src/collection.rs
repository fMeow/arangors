use std::sync::Arc;

use serde::{
    de::{Deserializer, Error as DeError},
    Deserialize, Serialize,
};
use serde_json::json;
use url::Url;

use maybe_async::maybe_async;

use crate::client::ClientExt;
use crate::response::ArangoResult;
use crate::{response::serialize_response, ClientError};

use super::{Database, Document};
use crate::document::{
    DocumentHeader, DocumentInsertOptions, DocumentOverwriteMode, DocumentReadOptions,
    DocumentResponse, DocumentUpdateOptions,
};
use http::{Method, Request};
use serde::de::DeserializeOwned;
use std::borrow::Borrow;
use std::fmt::Debug;

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
    /// If true then creating or changing a document will wait until the data has been synchronized to disk.
    pub wait_for_sync: Option<bool>,
    // for ArangoDb 3.7
    // TODO need to implement this with feature gate between versions maybe
    // pub schema: Option<SchemaRules>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionInfo {
    pub count: Option<u32>,
    pub id: String,
    pub name: String,
    pub globally_unique_id: String,
    pub is_system: bool,
    pub status: u16,
    pub r#type: CollectionType,
}

#[derive(Debug, Clone)]
pub struct Collection<'a, C: ClientExt> {
    id: String,
    name: String,
    collection_type: CollectionType,
    /// Collection url: http://server:port/_db/mydb/_api/collection/{collection-name}
    /// This url is used to work on the collection itself
    base_url: Url,
    /// Document base url: http://server:port/_db/mydb/_api/document/{collection-name}
    /// This url is used to work with documents
    document_base_url: Url,
    session: Arc<C>,
    phantom: &'a (),
}

impl<'a, C: ClientExt> Collection<'a, C> {
    /// Construct Collection given
    /// Base url should be like `http://server:port/_db/mydb/_api/collection/{collection-name}`
    /// Document root should be like: http://server:port/_db/mydb/_api/document/
    pub(crate) fn new<T: Into<String>>(
        database: &'a Database<C>,
        name: T,
        id: T,
        collection_type: CollectionType,
    ) -> Collection<'a, C> {
        let name = name.into();
        let path = format!("_api/collection/{}/", name.as_str());
        let url = database.get_url().join(path.as_str()).unwrap();
        let document_path = format!("_api/document/{}/", name.as_str());
        let document_base_url = database.get_url().join(document_path.as_str()).unwrap();
        Collection {
            name,
            id: id.into(),
            session: database.get_session(),
            base_url: url,
            document_base_url,
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
    #[maybe_async]
    pub async fn properties(&self) -> Result<CollectionProperties, ClientError> {
        let url = self.base_url.join("properties").unwrap();
        let resp: CollectionProperties =
            serialize_response(self.session.get(url, "").await?.text())?;
        Ok(resp)
    }

    /// Counts the documents in this collection
    ///
    #[maybe_async]
    pub async fn document_count(&self) -> Result<CollectionProperties, ClientError> {
        let url = self.base_url.join("count").unwrap();
        let resp: CollectionProperties =
            serialize_response(self.session.get(url, "").await?.text())?;
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
    #[maybe_async]
    pub async fn statistics(&self) -> Result<CollectionStatistics, ClientError> {
        let url = self.base_url.join("figures").unwrap();
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
    #[maybe_async]
    pub async fn revision_id(&self) -> Result<CollectionRevision, ClientError> {
        let url = self.base_url.join("revision").unwrap();
        let resp: CollectionRevision = serialize_response(self.session.get(url, "").await?.text())?;
        Ok(resp)
    }
    /// Fetch a checksum for the specified collection
    ///
    /// Will calculate a checksum of the meta-data (keys and optionally
    /// revision ids) and optionally the document data in the collection.
    /// The checksum can be used to compare if two collections on different ArangoDB
    /// instances contain the same contents. The current revision of the collection
    /// is returned too so one can make sure the checksums are calculated for the
    /// same state of data.
    ///
    /// By default, the checksum will only be calculated on the _key system
    /// attribute of the documents contained in the collection. For edge
    /// collections, the system attributes _from and _to will also be included in
    /// the calculation.
    ///
    /// By setting the optional query parameter withRevisions to true, then revision
    /// ids (_rev system attributes) are included in the checksumming.
    ///
    /// By providing the optional query parameter withData with a value of true, the
    /// user-defined document attributes will be included in the calculation too.
    /// Note: Including user-defined attributes will make the checksumming slower.
    /// this function would make a request to arango server.
    #[maybe_async]
    pub async fn checksum(&self) -> Result<CollectionChecksum, ClientError> {
        self.checksum_with_options(false, false).await
    }

    /// By setting the optional query parameter withRevisions to true, then revision
    /// ids (_rev system attributes) are included in the checksumming.
    ///
    /// By providing the optional query parameter withData with a value of true, the
    /// user-defined document attributes will be included in the calculation too.
    /// Note: Including user-defined attributes will make the checksumming slower.
    #[maybe_async]
    pub async fn checksum_with_options(
        &self,
        with_revisions: bool,
        with_data: bool,
    ) -> Result<CollectionChecksum, ClientError> {
        let mut url = self.base_url.join("checksum").unwrap();

        if with_revisions {
            url.query_pairs_mut().append_pair("withRevisions", "true");
        }
        if with_data {
            url.query_pairs_mut().append_pair("withData", "true");
        }

        let resp: CollectionChecksum = serialize_response(self.session.get(url, "").await?.text())?;
        Ok(resp)
    }

    /// Loads a collection into memory. Returns the collection on success.
    ///
    /// The request body object might optionally contain the following attribute:
    /// - count: If set, this controls whether the return value should include the number of documents in the collection.
    /// Setting count to false may speed up loading a collection. The default value for count is true.
    #[maybe_async]
    pub async fn load(&self, count: bool) -> Result<CollectionInfo, ClientError> {
        let url = self.base_url.join("load").unwrap();
        let body = json!({ "count": count });
        let resp: CollectionInfo = serialize_response(
            self.session
                .put(url, body.to_string().as_str())
                .await?
                .text(),
        )?;
        Ok(resp)
    }

    /// Removes a collection from memory. This call does not delete any
    /// documents. You can use the collection afterwards; in which case it will
    /// be loaded into memory, again.
    #[maybe_async]
    pub async fn unload(&self) -> Result<CollectionInfo, ClientError> {
        let url = self.base_url.join("unload").unwrap();
        let resp: CollectionInfo = serialize_response(self.session.put(url, "").await?.text())?;
        Ok(resp)
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
    /// On success this function returns an object with attribute result set to
    /// true
    #[maybe_async]
    pub async fn load_indexes(&self) -> Result<bool, ClientError> {
        let url = self.base_url.join("loadIndexesIntoMemory").unwrap();
        let resp: ArangoResult<bool> = serialize_response(self.session.put(url, "").await?.text())?;
        Ok(resp.unwrap())
    }

    /// Changes the properties of a collection.
    #[maybe_async]
    pub async fn change_properties(
        &self,
        properties: CollectionPropertiesOptions,
    ) -> Result<CollectionProperties, ClientError> {
        let url = self.base_url.join("properties").unwrap();
        let mut body = json!({});
        if properties.wait_for_sync.is_some() {
            body["waitForSync"] = json!(properties.wait_for_sync.unwrap());
        }
        let resp: CollectionProperties = serialize_response(
            self.session
                .put(url, body.to_string().as_str())
                .await?
                .text(),
        )?;
        Ok(resp)
    }

    /// Renames the collection
    #[maybe_async]
    pub async fn rename(&self, name: &str) -> Result<CollectionInfo, ClientError> {
        let url = self.base_url.join("rename").unwrap();
        let body = json!({ "name": name });
        let resp: CollectionInfo = serialize_response(
            self.session
                .put(url, body.to_string().as_str())
                .await?
                .text(),
        )?;
        Ok(resp)
    }

    /// recalculates the document count of a collection
    /// Note: this method is specific for the RocksDB storage engine
    #[cfg(feature = "rocksdb")]
    #[maybe_async]
    pub async fn recalculate_count(&self) -> Result<bool, ClientError> {
        let url = self.base_url.join("recalculateCount").unwrap();
        let resp: ArangoResult<bool> = serialize_response(self.session.put(url, "").await?.text())?;
        Ok(resp.unwrap())
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
    /// This methods is not documented on 3.7
    /// Note: this method is specific for the MMFiles storage engine, and there it is not available in a cluster.
    #[cfg(feature = "mmfiles")]
    #[maybe_async]
    pub async fn rotate_journal(&self) -> Result<bool, ClientError> {
        let url = self.base_url.join("rotate").unwrap();
        let resp: ArangoResult<bool> = serialize_response(self.session.put(url, "").await?.text())?;
        Ok(resp.unwrap())
    }

    /// Creates a new document from the document given in the body, unless
    /// there is already a document with the _key given. If no _key is given, a
    /// new unique _key is generated automatically.
    /// Possibly given _id and _rev attributes in the body are always ignored,
    /// the URL part or the query parameter collection respectively counts.
    ///
    /// If the document was created successfully, then the Location header contains
    /// the path to the newly created document.
    /// The Etag header field contains the revision of the document.
    /// Both are only set in the single document case.
    ///
    /// If silent is not set to true, the body of the response contains a JSON object with the following attributes:
    ///
    /// _id contains the document identifier of the newly created document
    /// _key contains the document key
    /// _rev contains the document revision
    /// If the collection parameter waitForSync is false, then the call returns as soon as the document has been accepted.
    /// It will not wait until the documents have been synced to disk.
    ///
    /// Optionally, the query parameter waitForSync can be used to force synchronization of the document creation
    /// operation to disk even in case that the waitForSync flag had been disabled for the entire collection.
    /// Thus, the waitForSync query parameter can be used to force synchronization of just this specific operations.
    /// To use this, set the waitForSync parameter to true. If the waitForSync parameter is not specified or set to false,
    /// then the collection’s default waitForSync behavior is applied.
    /// The waitForSync query parameter cannot be used to disable synchronization for collections that have a default waitForSync value of true.
    ///
    /// If the query parameter returnNew is true, then, for each generated document, the complete new document is returned under the new attribute in the result.
    /// # Note
    /// this function would make a request to arango server.
    #[maybe_async]
    pub async fn create_document<T>(
        &self,
        doc: T,
        insert_options: Option<DocumentInsertOptions>,
    ) -> Result<DocumentResponse<T>, ClientError>
    where
        T: Serialize + DeserializeOwned,
    {
        let mut url = self.document_base_url.join("").unwrap();
        let body = serde_json::to_string(&doc)?;

        if let Some(options) = insert_options {
            if let Some(return_new) = options.borrow().return_new {
                url.query_pairs_mut()
                    .append_pair("returnNew", return_new.to_string().as_str());
            }
            if let Some(wait_for_sync) = options.borrow().wait_for_sync {
                url.query_pairs_mut()
                    .append_pair("waitForSync", wait_for_sync.to_string().as_str());
            }
            if let Some(return_old) = options.borrow().return_old {
                url.query_pairs_mut()
                    .append_pair("returnOld", return_old.to_string().as_str());
            }
            if let Some(silent) = options.borrow().silent {
                url.query_pairs_mut()
                    .append_pair("silent", silent.to_string().as_str());
            }
            if let Some(overwrite) = options.borrow().overwrite {
                url.query_pairs_mut()
                    .append_pair("overwrite", overwrite.to_string().as_str());
            }
            #[cfg(feature = "arango3_7")]
            if let Some(overwrite_mode) = options.overwrite_mode {
                let mode = match overwrite_mode {
                    DocumentOverwriteMode::Ignore => "ignore",
                    DocumentOverwriteMode::Replace => "replace",
                    DocumentOverwriteMode::Update => "update",
                    DocumentOverwriteMode::Conflict => "conflict",
                };
                url.query_pairs_mut().append_pair("overwriteMode", mode);
            }
        }

        let resp: DocumentResponse<T> =
            serde_json::from_str(self.session.post(url, body.as_str()).await?.text())?;
        Ok(resp)
    }

    /// Reads a single document
    ///
    /// # Note
    /// this function would make a request to arango server.
    #[maybe_async]
    pub async fn read_document<T>(&self, _key: &str) -> Result<Document<T>, ClientError>
    where
        T: Serialize + Debug + DeserializeOwned,
    {
        self.read_document_with_options(_key, None).await
    }

    #[maybe_async]
    pub async fn read_document_with_options<T>(
        &self,
        _key: &str,
        read_options: Option<DocumentReadOptions>,
    ) -> Result<Document<T>, ClientError>
    where
        T: Serialize + Debug + DeserializeOwned,
    {
        let url = self.document_base_url.join(_key).unwrap();
        let mut build = Request::get(url.to_string());

        if let Some(options) = read_options {
            let key_value = match options {
                DocumentReadOptions::IfNoneMatch(value) => ("If-None-Match".to_string(), value),

                DocumentReadOptions::IfMatch(value) => ("If-Match".to_string(), value),
            };

            build = build.header(key_value.0.as_str(), key_value.1.as_str());
        }

        let req = build.body("".to_string()).unwrap();
        let resp: Document<T> = serde_json::from_str(self.session.request(req).await?.text())?;
        Ok(resp)
    }

    /// Reads a single document header
    /// Like GET, but only returns the header fields and not the body. You can use this call to get the current revision of a document or check if the document was deleted.
    /// # Note
    /// this function would make a request to arango server.
    #[maybe_async]
    pub async fn read_document_header(&self, _key: &str) -> Result<DocumentHeader, ClientError> {
        self.read_document_header_with_options(_key, None).await
    }

    #[maybe_async]
    pub async fn read_document_header_with_options(
        &self,
        _key: &str,
        read_options: Option<DocumentReadOptions>,
    ) -> Result<DocumentHeader, ClientError>
where {
        let url = self.document_base_url.join(_key).unwrap();
        let mut build = Request::get(url.to_string());

        if let Some(options) = read_options {
            let key_value = match options {
                DocumentReadOptions::IfNoneMatch(value) => ("If-None-Match".to_string(), value),

                DocumentReadOptions::IfMatch(value) => ("If-Match".to_string(), value),
            };

            build = build.header(key_value.0.as_str(), key_value.1.as_str());
        }

        let req = build.body("".to_string()).unwrap();
        let resp: DocumentHeader = serde_json::from_str(self.session.request(req).await?.text())?;
        Ok(resp)
    }
    /// Partially updates the document
    ///
    /// # Note
    /// this function would make a request to arango server.
    #[maybe_async]
    pub async fn update_document<T>(
        &self,
        _key: &str,
        doc: T,
        update_options: Option<DocumentUpdateOptions>,
    ) -> Result<DocumentResponse<T>, ClientError>
    where
        T: Serialize + DeserializeOwned,
    {
        let mut url = self.document_base_url.join(_key).unwrap();
        let body = serde_json::to_string(&doc)?;
        if let Some(options) = update_options {
            if let Some(keep_null) = options.borrow().keep_null {
                url.query_pairs_mut()
                    .append_pair("keep_null", keep_null.to_string().as_str());
            }
            if let Some(return_new) = options.borrow().return_new {
                url.query_pairs_mut()
                    .append_pair("returnNew", return_new.to_string().as_str());
            }
            if let Some(wait_for_sync) = options.borrow().wait_for_sync {
                url.query_pairs_mut()
                    .append_pair("waitForSync", wait_for_sync.to_string().as_str());
            }
            if let Some(ignore_revs) = options.borrow().ignore_revs {
                url.query_pairs_mut()
                    .append_pair("ignore_revs", ignore_revs.to_string().as_str());
            }
            if let Some(return_old) = options.borrow().return_old {
                url.query_pairs_mut()
                    .append_pair("returnOld", return_old.to_string().as_str());
            }
            if let Some(silent) = options.borrow().silent {
                url.query_pairs_mut()
                    .append_pair("silent", silent.to_string().as_str());
            }
        }

        let resp: DocumentResponse<T> =
            serde_json::from_str(self.session.patch(url, body.as_str()).await?.text())?;
        Ok(resp)
    }

    /// Replaces the document
    ///
    /// # Note
    /// this function would make a request to arango server.
    #[maybe_async]
    pub async fn replace_document<T>(&self, doc: Document<T>) {
        unimplemented!()
    }

    /// Removes a document
    ///
    /// # Note
    /// this function would make a request to arango server.
    #[maybe_async]
    pub async fn remove_document<T>(&self, doc: Document<T>) {
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

#[derive(Debug, Clone, PartialEq)]
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
