//! Collection level operations
//!
//! This mod contains struct and type of colleciton info and management, as well
//! as document related operations.
use std::{convert::TryFrom, sync::Arc};

use http::Request;
use maybe_async::maybe_async;
use serde::{de::DeserializeOwned, Deserialize, Deserializer, Serialize};
use serde_json::json;
use url::Url;

use options::*;
use response::*;

use crate::{
    client::ClientExt,
    document::{
        options::{InsertOptions, ReadOptions, RemoveOptions, ReplaceOptions, UpdateOptions},
        response::DocumentResponse,
        Header,
    },
    response::{deserialize_response, ArangoResult},
    transaction::Transaction,
    ClientError,
};

use super::{Database, Document};

pub mod options;
pub mod response;

/// Represent a collection in Arango server that consists of documents/edges.
///
/// It is uniquely identified by its
/// collection identifier. It also has a unique name that clients should use to
/// identify and access it. Collections can be renamed. This will change the
/// collection name, but not the collection identifier.
///
/// Collections have a type
/// that is specified by the user when the collection is created. There are
/// currently two types: document and edge. The default type is document.
#[derive(Debug, Clone)]
pub struct Collection<C: ClientExt> {
    id: String,
    name: String,
    collection_type: CollectionType,
    base_url: Url,
    document_base_url: Url,
    session: Arc<C>,
}

impl<'a, C: ClientExt> Collection<C> {
    /// Construct Collection given collection info from server
    ///
    /// Base url should be like `http://server:port/_db/mydb/_api/collection/{collection-name}`
    /// Document root should be like: http://server:port/_db/mydb/_api/document/
    pub(crate) fn new<T: Into<String>, S: Into<String>>(
        name: T,
        id: S,
        collection_type: CollectionType,
        db_url: &Url,
        session: Arc<C>,
    ) -> Collection<C> {
        let name = name.into();
        let path = format!("_api/collection/{}/", &name);
        let url = db_url.join(&path).unwrap();
        let document_path = format!("_api/document/{}/", &name);
        let document_base_url = db_url.join(&document_path).unwrap();
        Collection {
            name,
            id: id.into(),
            session,
            base_url: url,
            document_base_url,
            collection_type,
        }
    }

    pub(crate) fn from_response(database: &Database<C>, collection: &Info) -> Collection<C> {
        Self::new(
            &collection.name,
            &collection.id,
            collection.collection_type,
            database.url(),
            database.session(),
        )
    }

    pub(crate) fn from_transaction_response(
        transaction: &Transaction<C>,
        collection: &Info,
    ) -> Collection<C> {
        Self::new(
            &collection.name,
            &collection.id,
            collection.collection_type,
            transaction.url(),
            transaction.session(),
        )
    }

    pub fn collection_type(&self) -> CollectionType {
        self.collection_type
    }

    /// The collection identifier
    ///
    /// A collection identifier lets you refer to a collection in a database. It
    /// is a string value and is unique within the database. Up to including
    /// ArangoDB 1.1, the collection identifier has been a client’s primary
    /// means to access collections. Starting with ArangoDB 1.2, clients should
    /// instead use a collection’s unique name to access a collection instead of
    /// its identifier. ArangoDB currently uses 64bit unsigned integer values to
    /// maintain collection ids internally. When returning collection ids to
    /// clients, ArangoDB will put them into a string to ensure the collection
    /// id is not clipped by clients that do not support big integers. Clients
    /// should treat the collection ids returned by ArangoDB as opaque strings
    /// when they store or use them locally.
    //
    // Note: collection ids have been returned as integers up to including ArangoDB
    // 1.1
    pub fn id(&self) -> &str {
        self.id.as_str()
    }

    /// The collection name
    ///
    /// A collection name identifies a collection in a database. It is a string
    /// and is unique within the database. Unlike the collection identifier it
    /// is supplied by the creator of the collection. The collection name must
    /// consist of letters, digits, and the _ (underscore) and - (dash)
    /// characters only. Please refer to Naming Conventions in ArangoDB for more
    /// information on valid collection names.
    pub fn name(&self) -> &str {
        &self.name.as_str()
    }

    /// Collection url: http://server:port/_db/mydb/_api/collection/{collection-name}
    ///
    /// This url is used to work on the collection itself
    pub fn url(&self) -> &Url {
        &self.base_url
    }
    /// Document base url: http://server:port/_db/mydb/_api/document/{collection-name}
    ///
    /// This url is used to work with documents
    pub fn doc_url(&self) -> &Url {
        &self.document_base_url
    }

    /// HTTP Client used to query the server
    pub fn session(&self) -> Arc<C> {
        Arc::clone(&self.session)
    }

    /// Get the db of current collection
    pub fn db(&self) -> Database<C> {
        // Base url should be like `http://server:port/_db/mydb/_api/collection/{collection-name}`
        let mut paths = self.base_url.path_segments().unwrap();
        // must be `_db`
        paths.next();
        // must be db name
        let name = paths.next().unwrap();
        Database::new(name, &self.url().join("/").unwrap(), self.session())
    }

    /// Drop a collection
    ///
    /// # Note
    /// this function would make a request to arango server.
    #[maybe_async]
    pub async fn drop(self) -> Result<String, ClientError> {
        let url = self.base_url.join("").unwrap();

        #[derive(Debug, Deserialize)]
        struct DropCollectionResponse {
            id: String,
        }

        let resp: DropCollectionResponse =
            deserialize_response(self.session.delete(url, "").await?.body())?;
        Ok(resp.id)
    }

    /// Truncate current collection
    ///
    /// # Note
    /// this function would make a request to arango server.
    #[maybe_async]
    pub async fn truncate(&self) -> Result<Info, ClientError> {
        let url = self.base_url.join("truncate").unwrap();
        let resp: Info = deserialize_response(self.session.put(url, "").await?.body())?;
        Ok(resp)
    }

    /// Fetch the properties of collection
    ///
    /// # Note
    /// this function would make a request to arango server.
    #[maybe_async]
    pub async fn properties(&self) -> Result<Properties, ClientError> {
        let url = self.base_url.join("properties").unwrap();
        let resp: Properties = deserialize_response(self.session.get(url, "").await?.body())?;
        Ok(resp)
    }

    /// Count the documents in this collection
    ///
    /// # Note
    /// this function would make a request to arango server.
    #[maybe_async]
    pub async fn document_count(&self) -> Result<Properties, ClientError> {
        let url = self.base_url.join("count").unwrap();
        let resp: Properties = deserialize_response(self.session.get(url, "").await?.body())?;
        Ok(resp)
    }
    /// Fetch the statistics of a collection
    ///
    /// The result also contains the number of documents and additional
    /// statistical information about the collection. **Note**: This will
    /// always load the collection into memory.
    ///
    /// **Note**: collection data that are stored in the write-ahead log only
    /// are not reported in the results. When the write-ahead log is
    /// collected, documents might be added to journals and datafiles of
    /// the collection, which may modify the figures of the collection.
    ///
    /// Additionally, the file sizes of collection and index parameter JSON
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
    pub async fn statistics(&self) -> Result<Statistics, ClientError> {
        let url = self.base_url.join("figures").unwrap();
        let resp: Statistics = deserialize_response(self.session.get(url, "").await?.body())?;
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
    pub async fn revision_id(&self) -> Result<Revision, ClientError> {
        let url = self.base_url.join("revision").unwrap();
        let resp: Revision = deserialize_response(self.session.get(url, "").await?.body())?;
        Ok(resp)
    }
    /// Fetch a checksum for the specified collection
    ///
    /// Will calculate a checksum of the meta-data (keys and optionally
    /// revision ids) and optionally the document data in the collection.
    /// The checksum can be used to compare if two collections on different
    /// ArangoDB instances contain the same contents. The current revision
    /// of the collection is returned too so one can make sure the checksums
    /// are calculated for the same state of data.
    ///
    /// By default, the checksum will only be calculated on the _key system
    /// attribute of the documents contained in the collection. For edge
    /// collections, the system attributes _from and _to will also be included
    /// in the calculation.
    ///
    /// # Note
    /// this function would make a request to arango server.
    #[maybe_async]
    pub async fn checksum(&self) -> Result<Checksum, ClientError> {
        self.checksum_with_options(Default::default()).await
    }

    /// Fetch a checksum for the specified collection
    ///
    /// Will calculate a checksum of the meta-data (keys and optionally
    /// revision ids) and optionally the document data in the collection.
    /// The checksum can be used to compare if two collections on different
    /// ArangoDB instances contain the same contents. The current revision
    /// of the collection is returned too so one can make sure the checksums
    /// are calculated for the same state of data.
    ///
    /// By default, the checksum will only be calculated on the _key system
    /// attribute of the documents contained in the collection. For edge
    /// collections, the system attributes _from and _to will also be included
    /// in the calculation.
    ///
    /// By setting the optional query parameter withRevisions to true, then
    /// revision ids (_rev system attributes) are included in the
    /// checksumming.
    ///
    /// By providing the optional query parameter withData with a value of true,
    /// the user-defined document attributes will be included in the
    /// calculation too.
    ///
    /// Note: Including user-defined attributes will make
    /// the checksumming slower. this function would make a request to
    /// arango server.
    ///
    /// # Note
    /// this function would make a request to arango server.
    #[maybe_async]
    pub async fn checksum_with_options(
        &self,
        options: ChecksumOptions,
    ) -> Result<Checksum, ClientError> {
        let mut url = self.base_url.join("checksum").unwrap();
        let query = serde_qs::to_string(&options).unwrap();
        url.set_query(Some(query.as_str()));

        let resp: Checksum = deserialize_response(self.session.get(url, "").await?.body())?;
        Ok(resp)
    }

    /// Load a collection into memory
    ///
    /// Returns the collection on success.
    ///
    /// The request body object might optionally contain the following
    /// attribute:
    ///
    /// - count
    ///
    ///   If set, this controls whether the return value should include
    ///   the number of documents in the collection. Setting count to false may
    ///   speed up loading a collection. The default value for count is true.
    ///
    /// # Note
    /// this function would make a request to arango server.
    #[maybe_async]
    pub async fn load(&self, count: bool) -> Result<Info, ClientError> {
        let url = self.base_url.join("load").unwrap();
        let body = json!({ "count": count });
        let resp: Info =
            deserialize_response(self.session.put(url, body.to_string()).await?.body())?;
        Ok(resp)
    }

    /// Remove a collection from memory
    ///
    /// This call does not delete any documents. You can use the collection
    /// afterwards; in which case it will be loaded into memory, again.
    ///
    /// # Note
    /// this function would make a request to arango server.
    #[maybe_async]
    pub async fn unload(&self) -> Result<Info, ClientError> {
        let url = self.base_url.join("unload").unwrap();
        let resp: Info = deserialize_response(self.session.put(url, "").await?.body())?;
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
    ///
    /// # Note
    /// this function would make a request to arango server.
    #[maybe_async]
    pub async fn load_indexes(&self) -> Result<bool, ClientError> {
        let url = self.base_url.join("loadIndexesIntoMemory").unwrap();
        let resp: ArangoResult<bool> =
            deserialize_response(self.session.put(url, "").await?.body())?;
        Ok(resp.unwrap())
    }

    /// Change the properties of a collection
    ///
    /// # Note
    /// this function would make a request to arango server.
    #[maybe_async]
    pub async fn change_properties(
        &self,
        properties: PropertiesOptions,
    ) -> Result<Properties, ClientError> {
        let url = self.base_url.join("properties").unwrap();

        let body = serde_json::to_string(&properties).unwrap();
        let resp: Properties = deserialize_response(self.session.put(url, body).await?.body())?;
        Ok(resp)
    }

    /// Rename the collection
    ///
    /// # Note
    /// this function would make a request to arango server.
    #[maybe_async]
    pub async fn rename(&mut self, name: &str) -> Result<Info, ClientError> {
        let url = self.base_url.join("rename").unwrap();
        let body = json!({ "name": name });
        let resp: Info =
            deserialize_response(self.session.put(url, body.to_string()).await?.body())?;
        self.name = name.to_string();
        self.base_url = self.base_url.join(&format!("../{}/", name)).unwrap();
        Ok(resp)
    }

    /// Recalculate the document count of a collection
    ///
    /// **Note**: this method is specific for the RocksDB storage engine
    ///
    /// # Note
    /// this function would make a request to arango server.
    #[cfg(feature = "rocksdb")]
    #[maybe_async]
    pub async fn recalculate_count(&self) -> Result<bool, ClientError> {
        let url = self.base_url.join("recalculateCount").unwrap();
        let resp: ArangoResult<bool> =
            deserialize_response(self.session.put(url, "").await?.body())?;
        Ok(resp.unwrap())
    }
    /// Rotate the journal of a collection
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
    ///
    /// **Note**: this method is specific for the MMFiles storage engine, and
    /// there it is not available in a cluster.
    ///
    /// # Note
    /// this function would make a request to arango server.
    #[cfg(feature = "mmfiles")]
    #[maybe_async]
    pub async fn rotate_journal(&self) -> Result<bool, ClientError> {
        let url = self.base_url.join("rotate").unwrap();
        let resp: ArangoResult<bool> =
            deserialize_response(self.session.put(url, "").await?.body())?;
        Ok(resp.unwrap())
    }

    /// Create a new document from the document given in the body, unless
    /// there is already a document with the _key given.
    ///
    /// If no _key is given, a new unique _key is generated automatically.
    /// Possibly given _id and _rev attributes in the body are always ignored,
    /// the URL part or the query parameter collection respectively counts.
    ///
    /// If the document was created successfully, then the Location header
    /// contains the path to the newly created document.
    /// The Etag header field contains the revision of the document.
    /// Both are only set in the single document case.
    ///
    /// If silent is not set to true, the body of the response contains a JSON
    /// object with the following attributes:
    ///
    /// - _id contains the document identifier of the newly created document
    /// - _key contains the document key
    /// - _rev contains the document revision
    ///
    /// If the collection parameter waitForSync is false, then the call returns
    /// as soon as the document has been accepted. It will not wait until
    /// the documents have been synced to disk.
    ///
    /// Optionally, the query parameter waitForSync can be used to force
    /// synchronization of the document creation operation to disk even in
    /// case that the waitForSync flag had been disabled for the entire
    /// collection. Thus, the waitForSync query parameter can be used to
    /// force synchronization of just this specific operations. To use this,
    /// set the waitForSync parameter to true. If the waitForSync parameter is
    /// not specified or set to false, then the collection’s default
    /// waitForSync behavior is applied. The waitForSync query parameter
    /// cannot be used to disable synchronization for collections that have a
    /// default waitForSync value of true.
    ///
    /// If the query parameter returnNew is true, then, for each generated
    /// document, the complete new document is returned under the new attribute
    /// in the result.
    ///
    /// # Note
    /// this function would make a request to arango server.
    #[maybe_async]
    pub async fn create_document<T>(
        &self,
        doc: T,
        insert_options: InsertOptions,
    ) -> Result<DocumentResponse<T>, ClientError>
    where
        T: Serialize + DeserializeOwned,
    {
        let mut url = self.document_base_url.join("").unwrap();
        let body = serde_json::to_string(&doc)?;
        let query = serde_qs::to_string(&insert_options).unwrap();
        url.set_query(Some(query.as_str()));
        let resp: DocumentResponse<T> =
            deserialize_response(self.session.post(url, body).await?.body())?;
        Ok(resp)
    }

    /// Read a single document with `_key`
    ///
    /// Returns the document identified by document-id. The returned document
    /// contains three special attributes: _id containing the document
    /// identifier, _key containing key which uniquely identifies a document in
    /// a given collection and _rev containing the revision.
    ///
    /// # Note
    /// this function would make a request to arango server.
    #[maybe_async]
    pub async fn document<T>(&self, _key: &str) -> Result<Document<T>, ClientError>
    where
        T: Serialize + DeserializeOwned,
    {
        self.document_with_options(_key, Default::default()).await
    }

    /// Read a single document with options
    ///
    /// Returns the document identified by document-id. The returned document
    /// contains three special attributes: _id containing the document
    /// identifier, _key containing key which uniquely identifies a document in
    /// a given collection and _rev containing the revision.
    ///
    /// # Note
    /// this function would make a request to arango server.
    #[maybe_async]
    pub async fn document_with_options<T>(
        &self,
        _key: &str,
        read_options: ReadOptions,
    ) -> Result<Document<T>, ClientError>
    where
        T: Serialize + DeserializeOwned,
    {
        let url = self.document_base_url.join(_key).unwrap();
        let mut build = Request::get(url.to_string());

        let header = make_header_from_options(read_options);
        if let Some(h) = header {
            build = build.header(h.0, h.1)
        }
        let req = build.body("".to_string()).unwrap();
        let resp: Document<T> = deserialize_response(self.session.request(req).await?.body())?;
        Ok(resp)
    }

    /// Read a single document header
    ///
    /// Like GET, but only returns the header fields and not the body. You can
    /// use this call to get the current revision of a document or check if the
    /// document was deleted.
    ///
    /// # Note
    /// this function would make a request to arango server.
    #[maybe_async]
    pub async fn document_header(&self, _key: &str) -> Result<Header, ClientError> {
        self.document_header_with_options(_key, Default::default())
            .await
    }

    /// Read a single document header with options
    ///
    /// Like GET, but only returns the header fields and not the body. You can
    /// use this call to get the current revision of a document or check if the
    /// document was deleted.
    ///
    /// # Note
    /// this function would make a request to arango server.
    #[maybe_async]
    pub async fn document_header_with_options(
        &self,
        _key: &str,
        read_options: ReadOptions,
    ) -> Result<Header, ClientError> {
        let url = self.document_base_url.join(_key).unwrap();
        let mut build = Request::get(url.to_string());

        let header = make_header_from_options(read_options);
        if let Some(h) = header {
            build = build.header(h.0, h.1)
        }
        let req = build.body("".to_string()).unwrap();
        let resp: Header = deserialize_response(self.session.request(req).await?.body())?;
        Ok(resp)
    }
    /// Partially update a document
    ///
    /// # Note
    /// this function would make a request to arango server.
    #[maybe_async]
    pub async fn update_document<T>(
        &self,
        _key: &str,
        doc: T,
        update_options: UpdateOptions,
    ) -> Result<DocumentResponse<T>, ClientError>
    where
        T: Serialize + DeserializeOwned,
    {
        let mut url = self.document_base_url.join(_key).unwrap();
        let body = serde_json::to_string(&doc)?;
        let query = serde_qs::to_string(&update_options).unwrap();
        url.set_query(Some(query.as_str()));

        let resp: DocumentResponse<T> =
            deserialize_response(self.session.patch(url, body).await?.body())?;
        Ok(resp)
    }

    /// Replace a document
    ///
    /// Replaces the specified document with the one in the body, provided there
    /// is such a document and no precondition is violated.
    ///
    /// The value of the _key attribute as well as attributes used as sharding
    /// keys may not be changed.
    ///
    /// If the If-Match header is specified and the revision of the document in
    /// the database is unequal to the given revision, the precondition is
    /// violated. If If-Match is not given and ignoreRevs is false and there
    /// is a _rev attribute in the body and its value does not match the
    /// revision of the document in the database, the precondition is violated.
    /// If a precondition is violated, an HTTP 412 is returned.
    ///
    /// If the document exists and can be updated, then an HTTP 201 or an HTTP
    /// 202 is returned (depending on waitForSync, see below), the Etag header
    /// field contains the new revision of the document and the Location header
    /// contains a complete URL under which the document can be queried.
    ///
    /// Cluster only: The replace documents may contain values for the
    /// collection’s pre-defined shard keys. Values for the shard keys are
    /// treated as hints to improve performance. Should the shard keys values be
    /// incorrect ArangoDB may answer with a not found error. Optionally,
    /// the query parameter waitForSync can be used to force synchronization of
    /// the document replacement operation to disk even in case that the
    /// waitForSync flag had been disabled for the entire collection. Thus, the
    /// waitForSync query parameter can be used to force synchronization of just
    /// specific operations. To use this, set the waitForSync parameter to
    /// true. If the waitForSync parameter is not specified or set to false,
    /// then the collection’s default waitForSync behavior is applied. The
    /// waitForSync query parameter cannot be used to disable synchronization
    /// for collections that have a default waitForSync value of true.
    ///
    /// If silent is not set to true, the body of the response contains a JSON
    /// object with the information about the identifier and the revision. The
    /// attribute _id contains the known document-id of the updated document,
    /// _key contains the key which uniquely identifies a document in a given
    /// collection, and the attribute _rev contains the new document revision.
    ///
    /// If the query parameter returnOld is true, then the complete previous
    /// revision of the document is returned under the old attribute in the
    /// result. If the query parameter returnNew is true, then the complete
    /// new document is returned under the new attribute in the result.
    ///
    /// If the document does not exist, then a HTTP 404 is returned and the body
    /// of the response contains an error document.
    ///
    /// You can conditionally replace a document based on a target revision id
    /// by using the if-match HTTP header.
    ///
    /// # Note
    /// this function would make a request to arango server.
    #[maybe_async]
    pub async fn replace_document<T>(
        &self,
        _key: &str,
        doc: T,
        replace_options: ReplaceOptions,
        if_match_header: Option<String>,
    ) -> Result<DocumentResponse<T>, ClientError>
    where
        T: Serialize + DeserializeOwned,
    {
        let mut url = self.document_base_url.join(_key).unwrap();
        let body = serde_json::to_string(&doc)?;
        let query = serde_qs::to_string(&replace_options).unwrap();
        url.set_query(Some(query.as_str()));

        let mut build = Request::put(url.to_string());

        if let Some(if_match_value) = if_match_header {
            build = build.header("If-Match", if_match_value);
        }

        let req = build.body(body).unwrap();

        let resp: DocumentResponse<T> =
            deserialize_response(self.session.request(req).await?.body())?;
        Ok(resp)
    }

    /// Remove a document
    ///
    /// If silent is not set to true, the body of the response contains a JSON
    /// object with the information about the identifier and the revision. The
    /// attribute _id contains the known document-id of the removed document,
    /// _key contains the key which uniquely identifies a document in a given
    /// collection, and the attribute _rev contains the document revision.
    ///
    /// If the waitForSync parameter is not specified or set to false, then the
    /// collection’s default waitForSync behavior is applied. The
    /// waitForSync query parameter cannot be used to disable
    /// synchronization for collections that have a default waitForSync value of
    /// true.
    ///
    /// If the query parameter returnOld is true, then the complete previous
    /// revision of the document is returned under the old attribute in the
    /// result.
    ///
    /// You can conditionally replace a document based on a target revision id
    /// by using the if-match HTTP header.
    ///
    /// # Note
    /// this function would make a request to arango server.
    #[maybe_async]
    pub async fn remove_document<T>(
        &self,
        _key: &str,
        remove_options: RemoveOptions,
        if_match_header: Option<String>,
    ) -> Result<DocumentResponse<T>, ClientError>
    where
        T: Serialize + DeserializeOwned,
    {
        let mut url = self.document_base_url.join(_key).unwrap();
        let query = serde_qs::to_string(&remove_options).unwrap();
        url.set_query(Some(query.as_str()));

        let mut build = Request::delete(url.to_string());

        if let Some(if_match_value) = if_match_header {
            build = build.header("If-Match", if_match_value);
        }

        let req = build.body("".to_string()).unwrap();

        let resp: DocumentResponse<T> =
            deserialize_response(self.session.request(req).await?.body())?;
        Ok(resp)
    }

    /// Returns a new Collection with its `session` updated with the transaction id
    pub fn clone_with_transaction(&self, transaction_id: String) -> Result<Self, ClientError> {
        Ok(Self {
            session: Arc::new(self.session.clone_with_transaction(transaction_id)?),
            ..self.clone()
        })
    }
}

/// Create header name and header value from read_options
fn make_header_from_options(
    document_read_options: ReadOptions,
) -> Option<(http::header::HeaderName, http::header::HeaderValue)> {
    match document_read_options {
        ReadOptions::IfNoneMatch(value) => Some((
            "If-None-Match".to_string().parse().unwrap(),
            http::HeaderValue::try_from(value).unwrap(),
        )),

        ReadOptions::IfMatch(value) => Some((
            "If-Match".to_string().parse().unwrap(),
            http::HeaderValue::try_from(value).unwrap(),
        )),

        ReadOptions::NoHeader => None,
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Serialize)]
pub enum CollectionType {
    #[serde(rename = "2")]
    Document = 2,
    #[serde(rename = "3")]
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
            _ => Err(serde::de::Error::custom(
                "Undefined behavior. If the crate breaks after an upgrade of ArangoDB, please \
                 contact the author.",
            )),
        }
    }
}
