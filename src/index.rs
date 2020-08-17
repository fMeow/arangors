//! Represents the type of an [`Index`] in ArangoDB. The following types are
//! supported:
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

/// Represents the type of an [`Index`]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Type {
    Primary,
    Fulltext,
    Geo,
    Hash,
    Persistent,
    Skiplist,
    Ttl,
}

/// Represents an index on a collection in ArangoDB.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Index {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_newly_created: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    selectivity_estimate: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<u16>,
    fields: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unique: Option<bool>,
    r#type: Type,
    #[serde(skip_serializing_if = "Option::is_none")]
    sparse: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_length: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deduplicate: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    geo_json: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expire_after: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    in_background: Option<bool>,
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

impl Index {
    fn default(r#type: Type) -> Self {
        Index {
            id: None,
            is_newly_created: None,
            selectivity_estimate: None,
            error: None,
            code: None,
            fields: vec![],
            name: None,
            unique: None,
            r#type,
            sparse: None,
            min_length: None,
            deduplicate: None,
            geo_json: None,
            expire_after: None,
            in_background: Some(false),
        }
    }

    /// Creates a persistent index. The index can be applied to a collection using [`database::create_index`].
    ///
    /// # Arguments
    ///
    /// * `fields` - A vector of strings that holds the field names to be included in the index
    /// * `unique` - Whether or not to enforce uniqueness for this index
    /// * `sparse` - Create a sparse index (exclude documents that do not have all the fields set)
    /// * `deduplicate` - Controls whether inserting duplicate index values from the same document into a unique array index will lead to a unique constraint error or not
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Create a persistent index with unique values.
    /// let idx = Index::persistent(vec!["emailAddress".to_string()], true, false, false);
    ///
    /// // Get the database
    /// let mut database = conn.db("test_db").await.expect("Could not connect to database");
    ///
    /// // Create the index
    /// let idx = database
    ///         .create_index(collection_name, &index)
    ///         .await
    ///         .unwrap();
    /// ```
    pub fn persistent(fields: Vec<String>, unique: bool, sparse: bool, deduplicate: bool) -> Self {
        let mut index = Index::default(Type::Persistent);
        index.fields = fields;
        index.unique = Some(unique);
        index.sparse = Some(sparse);
        index.deduplicate = Some(deduplicate);
        index
    }

    /// Creates a hash index. The index can be applied to a collection using [`database::create_index`].
    pub fn hash(fields: Vec<String>, unique: bool, sparse: bool, deduplicate: bool) -> Self {
        let mut index = Index::default(Type::Hash);
        index.fields = fields;
        index.unique = Some(unique);
        index.sparse = Some(sparse);
        index.deduplicate = Some(deduplicate);
        index
    }

    /// Creates a skiplist index. The index can be applied to a collection using [`database::create_index`].
    pub fn skip_list(fields: Vec<String>, unique: bool, sparse: bool, deduplicate: bool) -> Self {
        let mut index = Index::default(Type::Skiplist);
        index.fields = fields;
        index.unique = Some(unique);
        index.sparse = Some(sparse);
        index.deduplicate = Some(deduplicate);
        index
    }

    /// Creates a time to live index. The index can be applied to a collection using [`database::create_index`].
    pub fn ttl(fields: Vec<String>, expire_after: u32) -> Self {
        let mut index = Index::default(Type::Ttl);
        index.fields = fields;
        index.expire_after = Some(expire_after);
        index
    }

    /// Creates a geo index. The index can be applied to a collection using [`database::create_index`].
    pub fn geo(fields: Vec<String>) -> Self {
        let mut index = Index::default(Type::Geo);
        index.fields = fields;
        index
    }

    /// Set's the name of an index.
    pub fn with_name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Creates the index in the background, not exclusively and fully locking the collection
    /// on which the index is being built.
    pub fn create_in_background(mut self) -> Self {
        self.in_background = Some(true);
        self
    }

    /// Returns the id of the index.
    pub fn id(&self) -> &Option<String> {
        &self.id
    }

    /// Returns `&Some(true)` if the index has been newly created, `&Some(false)` otherwise.
    pub fn is_newly_created(&self) -> &Option<bool> {
        &self.is_newly_created
    }

    pub fn selectivity_estimate(&self) -> &Option<u8> {
        &self.selectivity_estimate
    }

    /// Indicates for errors on the index.
    pub fn error(&self) -> &Option<bool> {
        &self.error
    }

    /// The return code (for example `201` for a newly created index).
    pub fn code(&self) -> &Option<u16> {
        &self.code
    }

    /// Returns the name of an index.
    pub fn name(&self) -> &Option<String> {
        &self.name
    }

    /// Returns whether or not the index has the `unique` flag set.
    pub fn unique(&self) -> &Option<bool> {
        &self.unique
    }

    /// Returns whether or not the index has the `sparse` flag set.
    pub fn sparse(&self) -> &Option<bool> {
        &self.sparse
    }

    /// Returns whether or not the index has the `deduplicate` flag set.
    pub fn deduplicate(&self) -> &Option<bool> {
        &self.deduplicate
    }

    /// Returns the `min_length` property. Only valid for Fulltext index.
    pub fn min_length(&self) -> &Option<u32> {
        &self.min_length
    }

    /// Returns the `geo_json` property. Only valid for Geo index.
    pub fn geo_json(&self) -> &Option<bool> {
        &self.geo_json
    }

    /// Returns the `expire_after` property. Only valid for Ttl index.
    pub fn expire_after(&self) -> &Option<u32> {
        &self.expire_after
    }

    /// Returns if the index is being created in background.
    pub fn in_background(&self) -> &Option<bool> {
        &self.in_background
    }
}
