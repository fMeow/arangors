use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// Options for document insertion.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DocumentInsertOptions {
    /// Wait until document has been synced to disk.
    pub wait_for_sync: Option<bool>,
    /// Additionally return the complete new document under the attribute new in the result.
    pub return_new: Option<bool>,
    /// Additionally return the complete old document under the attribute old in the result. Only available if the overwrite option is used.
    pub return_old: Option<bool>,
    /// If set to true, an empty object will be returned as response.
    /// No meta-data will be returned for the created document.
    /// This option can be used to save some network traffic.
    pub silent: Option<bool>,
    /// If set to true, the insert becomes a replace-insert.
    /// If a document with the same _key already exists the new document is not rejected with unique constraint violated but will replace the old document.
    pub overwrite: Option<bool>,
    /// TODO add nice formatted documentation from official doc
    #[cfg(arango3_7)]
    pub overwrite_mode: Option<DocumentOverwriteMode>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DocumentOverwriteMode {
    Ignore,
    Replace,
    Update,
    Conflict,
}

/// Options for document reading.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DocumentReadOptions {
    /// If the “If-None-Match” header is given, then it must contain exactly one Etag.
    /// The document is returned, if it has a different revision than the given Etag. Otherwise an HTTP 304 is returned.
    pub if_none_match: Option<String>,
    ///  If the “If-Match” header is given, then it must contain exactly one Etag.
    /// The document is returned, if it has the same revision as the given Etag. Otherwise a HTTP 412 is returned.
    pub if_match: Option<String>,
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
pub struct DocumentHeaderOptions {
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
    pub new: Option<Document<T>>,
    #[serde(rename = "_oldRev")]
    pub _old_red: Option<String>,
    pub old: Option<Document<T>>,
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
    T: Serialize + Deserialize<'de> + Debug,
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
