use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// Options for document insertion.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DocumentInsertOptions {
    /// Wait until document has been synced to disk.
    pub wait_for_sync: Option<bool>,
    /// Additionally return the complete new document under the attribute new in
    /// the result.
    pub return_new: Option<bool>,
    /// Additionally return the complete old document under the attribute old in
    /// the result. Only available if the overwrite option is used.
    pub return_old: Option<bool>,
    /// If set to true, an empty object will be returned as response.
    /// No meta-data will be returned for the created document.
    /// This option can be used to save some network traffic.
    pub silent: Option<bool>,
    /// If set to true, the insert becomes a replace-insert.
    /// If a document with the same _key already exists the new document is not
    /// rejected with unique constraint violated but will replace the old
    /// document.
    pub overwrite: Option<bool>,
    /// TODO add nice formatted documentation from official doc
    #[cfg(arango3_7)]
    pub overwrite_mode: Option<DocumentOverwriteMode>,
}
/// Options for document update,
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DocumentUpdateOptions {
    pub keep_null: Option<bool>,
    pub merge_objects: Option<bool>,
    /// Wait until document has been synced to disk.
    pub wait_for_sync: Option<bool>,
    pub ignore_revs: Option<bool>,
    /// Additionally return the complete new document under the attribute new in
    /// the result.
    pub return_new: Option<bool>,
    /// Additionally return the complete old document under the attribute old in
    /// the result. Only available if the overwrite option is used.
    pub return_old: Option<bool>,
    /// If set to true, an empty object will be returned as response.
    /// No meta-data will be returned for the created document.
    /// This option can be used to save some network traffic.
    pub silent: Option<bool>,
}
#[derive(Serialize, Deserialize, Debug)]
pub enum DocumentOverwriteMode {
    /// If a document with the specified _key value exists already,
    /// nothing will be done and no write operation will be carried out.
    /// The insert operation will return success in this case.
    /// This mode does not support returning the old document version using
    /// RETURN OLD. When using RETURN NEW, null will be returned in case the
    /// document already existed.
    Ignore,
    /// If a document with the specified _key value exists already, it will be
    /// overwritten with the specified document value. This mode will also
    /// be used when no overwrite mode is specified but the overwrite flag is
    /// set to true.
    Replace,
    /// If a document with the specified _key value exists already, it will be
    /// patched (partially updated) with the specified document value.
    /// The overwrite mode can be further controlled via the keepNull and
    /// mergeObjects parameters
    Update,
    /// if a document with the specified _key value exists already, return a
    /// unique constraint violation error so that the insert operation fails.
    /// This is also the default behavior in case the overwrite mode is not set,
    /// and the overwrite flag is false or not set either.
    ///
    /// keepNull (optional): If the intention is to delete existing attributes
    /// with the update-insert command, the URL query parameter keepNull can be
    /// used with a value of false. This will modify the behavior of the patch
    /// command to remove any attributes from the existing document that are
    /// contained in the patch document with an attribute value of null.
    /// This option controls the update-insert behavior only.
    ///
    /// mergeObjects (optional): Controls whether objects (not arrays) will be
    /// merged if present in both the existing and the update-insert document.
    /// If set to false, the value in the patch document will overwrite the
    /// existing document’s value. If set to true, objects will be merged. The
    /// default is true. This option controls the update-insert behavior only.
    /// TODO need to implement the two extra modes keepNull & mergeObjects
    Conflict,
}

/// Options for document reading.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum DocumentReadOptions {
    /// If the “If-None-Match” header is given, then it must contain exactly one
    /// Etag. The document is returned, if it has a different revision than
    /// the given Etag. Otherwise an HTTP 304 is returned.
    IfNoneMatch(String),
    ///  If the “If-Match” header is given, then it must contain exactly one
    /// Etag. The document is returned, if it has the same revision as the
    /// given Etag. Otherwise a HTTP 412 is returned.
    IfMatch(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentHeader {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub _id: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub _key: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub _rev: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentResponse<T> {
    #[serde(flatten)]
    pub header: Option<DocumentHeader>,
    pub new: Option<T>,
    #[serde(rename = "_oldRev")]
    pub _old_rev: Option<String>,
    pub old: Option<T>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Document<T> {
    #[serde(flatten)]
    pub header: DocumentHeader,
    #[serde(flatten)]
    pub document: T,
}

impl<'de, T> Document<T>
where
    T: Serialize + Deserialize<'de>,
{
    pub fn new(data: T) -> Self {
        Document {
            document: data,
            header: DocumentHeader {
                _id: String::new(),
                _key: String::new(),
                _rev: String::new(),
            },
        }
    }
}
