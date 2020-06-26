use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]

/// Options for document insertion.
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
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Document<T> {
    #[serde(skip_serializing_if = "String::is_empty")]
    _id: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    _key: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    _rev: String,

    #[serde(flatten)]
    pub document: T,
}

impl<T> Document<T> {
    pub fn new(data: T) -> Self {
        Document {
            document: data,
            _id: String::new(),
            _key: String::new(),
            _rev: String::new(),
        }
    }
}
