use failure::Error;
use std::rc::Rc;

use reqwest::{Client, Url};
use serde::de::{Deserialize, Deserializer, Error as DeError};
use serde_derive::Deserialize;

use super::{Database, Document};

#[derive(Debug)]
pub struct Collection {
    id: String,
    name: String,
    collection_type: CollectionType,
    base_url: Url,
    session: Rc<Client>,
}
impl<'a, 'b: 'a> Collection {
    /// Construct Collection given
    ///  Base url should be like `http://localhost:8529/_db/mydb/_api/`
    pub(crate) fn new<T: Into<String>>(
        database: &'b Database,
        name: T,
        id: T,
        collection_type: CollectionType,
    ) -> Result<Collection, Error> {
        let name = name.into();
        let path = format!("collection/{}/", name.as_str());
        let url = database.get_url().join(path.as_str())?;
        Ok(Collection {
            name: name,
            id: id.into(),
            session: database.get_session(),
            base_url: url,
            collection_type,
        })
    }

    pub(crate) fn from_response(
        database: &'b Database,
        collection: &CollectionResponse,
    ) -> Result<Collection, Error> {
        Self::new(
            database,
            collection.name.to_owned(),
            collection.id.to_owned(),
            collection.collection_type.clone(),
        )
    }

    pub fn get_collection_type(&self) -> &CollectionType {
        &self.collection_type
    }

    pub fn get_id(&self) -> &str {
        self.id.as_str()
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn get_url(&self) -> &Url {
        &self.base_url
    }

    pub fn get_session(&self) -> Rc<Client> {
        Rc::clone(&self.session)
    }

    pub fn truncate(&self) {
        unimplemented!()
    }

    /// Fetch the properties of collection
    pub fn fetch_properties(&self) {
        unimplemented!()
    }

    /// Counts the documents in this collection
    pub fn fetch_document_count(&self) {
        unimplemented!()
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
    pub fn fetch_statistics(&self) {
        unimplemented!()
    }

    /// Retrieve the collections revision id
    ///
    /// The revision id is a server-generated string that clients can use to
    /// check whether data in a collection has changed since the last revision
    /// check.
    pub fn fetch_revision_id(&self) {
        unimplemented!()
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
    pub fn fetch_checksum(&self) {
        unimplemented!()
    }

    /// Loads a collection into memory.
    pub fn load(&self) {
        unimplemented!()
    }
    /// Removes a collection from memory. This call does not delete any
    /// documents. You can use the collection afterwards; in which case it will
    /// be loaded into memory, again.
    pub fn unload(&self) {
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
    pub fn load_indexes(&self) {
        unimplemented!()
    }

    /// Changes the properties of a collection.
    pub fn change_properties(&self) {
        unimplemented!()
    }

    /// Renames the collection
    pub fn rename_collection(&self) {
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
    pub fn rotate_journal(&self) {
        unimplemented!()
    }

    /// Creates a new document from the document given in the body, unless
    /// there is already a document with the _key given. If no _key is given, a
    /// new unique _key is generated automatically.
    pub fn create_document<T>(&self, doc: Document<T>) {
        unimplemented!()
    }

    /// Partially updates the document
    pub fn update_document<T>(&self, doc: Document<T>) {
        unimplemented!()
    }

    /// Replaces the document
    pub fn replace_document<T>(&self, doc: Document<T>) {
        unimplemented!()
    }

    /// Removes a document
    pub fn remove_document<T>(&self, doc: Document<T>) {
        unimplemented!()
    }
}

#[derive(Debug, Deserialize)]
pub struct CollectionResponse {
    pub id: String,
    pub name: String,
    pub status: CollectionStatus,
    #[serde(rename = "type")]
    pub collection_type: CollectionType,
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
            _ => Err(DeError::custom("Undefined behavior. If the crate breaks after a upgrade of ArangoDB, please contact the author.")),
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
            _ => Err(DeError::custom("Undefined behavior. If the crate breaks after a upgrade of ArangoDB, please contact the author.")),
        }
    }
}
