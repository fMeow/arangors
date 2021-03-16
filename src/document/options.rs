//! Types of options related to document
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

/// Options for document insertion.
#[derive(Debug, Serialize, Deserialize, PartialEq, TypedBuilder)]
#[builder(doc)]
#[serde(rename_all = "camelCase")]
pub struct InsertOptions {
    /// Wait until document has been synced to disk.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    wait_for_sync: Option<bool>,
    /// Additionally return the complete new document under the attribute new in
    /// the result.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    return_new: Option<bool>,
    /// Additionally return the complete old document under the attribute old in
    /// the result. Only available if the overwrite option is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    return_old: Option<bool>,
    /// If set to true, an empty object will be returned as response.
    /// No meta-data will be returned for the created document.
    /// This option can be used to save some network traffic.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    silent: Option<bool>,
    /// If set to true, the insert becomes a replace-insert.
    /// If a document with the same _key already exists the new document is not
    /// rejected with unique constraint violated but will replace the old
    /// document.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    overwrite: Option<bool>,
    /// TODO add nice formatted documentation from official doc
    #[cfg(feature = "arango3_7")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    overwrite_mode: Option<OverwriteMode>,

    /// If the intention is to delete existing attributes with the update-insert command,
    /// the URL query parameter keepNull can be used with a value of false.
    /// This will modify the behavior of the patch command to remove any attributes
    /// from the existing document that are contained in the patch document with an
    /// attribute value of null. This option controls the update-insert behavior only.
    #[cfg(feature = "arango3_7")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    keep_null: Option<bool>,

    /// Controls whether objects (not arrays) will be merged if present in both the existing
    /// and the update-insert document.
    /// If set to false, the value in the patch document will overwrite the existing document’s value.
    /// If set to true, objects will be merged. The default is true.
    /// This option controls the update-insert behavior only.
    #[cfg(feature = "arango3_7")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    merge_objects: Option<bool>,
}

impl Default for InsertOptions {
    fn default() -> Self {
        Self::builder().build()
    }
}

/// Options for document update,
#[derive(Debug, Serialize, Deserialize, PartialEq, TypedBuilder)]
#[builder(doc)]
#[serde(rename_all = "camelCase")]
pub struct UpdateOptions {
    /// If the intention is to delete existing attributes with the patch
    /// command, the URL query parameter keepNull can be used with a value of
    /// false. This will modify the behavior of the patch command to remove any
    /// attributes from the existing document that are contained in the patch
    /// document with an attribute value of null.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    keep_null: Option<bool>,
    /// Controls whether objects (not arrays) will be merged if present in both
    /// the existing and the patch document. If set to false, the value in the
    /// patch document will overwrite the existing document’s value. If set to
    /// true, objects will be merged. The default is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    merge_objects: Option<bool>,
    /// Wait until document has been synced to disk.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    wait_for_sync: Option<bool>,
    /// By default, or if this is set to true, the _rev attributes in the given
    /// document is ignored. If this is set to false, then the _rev
    /// attribute given in the body document is taken as a precondition. The
    /// document is only update if the current revision is the one specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    ignore_revs: Option<bool>,
    /// Additionally return the complete new document under the attribute new in
    /// the result.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    return_new: Option<bool>,
    /// Return additionally the complete previous revision of the changed
    /// document under the attribute old in the result.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    return_old: Option<bool>,
    /// If set to true, an empty object will be returned as response.
    /// No meta-data will be returned for the updated document.
    /// This option can be used to save some network traffic.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    silent: Option<bool>,
}

impl Default for UpdateOptions {
    fn default() -> Self {
        Self::builder().build()
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum OverwriteMode {
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

/// Options for document replace,
#[derive(Debug, Serialize, Deserialize, TypedBuilder)]
#[builder(doc)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceOptions {
    /// Wait until document has been synced to disk.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    wait_for_sync: Option<bool>,
    /// By default, or if this is set to true, the _rev attributes in the given
    /// document is ignored. If this is set to false, then the _rev
    /// attribute given in the body document is taken as a precondition. The
    /// document is only replaced if the current revision is the one specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    ignore_revs: Option<bool>,
    /// Additionally return the complete new document under the attribute new in
    /// the result.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    return_new: Option<bool>,
    /// Additionally return the complete old document under the attribute old in
    /// the result.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    return_old: Option<bool>,
    /// If set to true, an empty object will be returned as response.
    /// No meta-data will be returned for the replaced document.
    /// This option can be used to save some network traffic.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    silent: Option<bool>,
}

impl Default for ReplaceOptions {
    fn default() -> Self {
        Self::builder().build()
    }
}

/// Options for document reading.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReadOptions {
    /// If the “If-None-Match” header is given, then it must contain exactly one
    /// Etag. The document is returned, if it has a different revision than
    /// the given Etag. Otherwise an HTTP 304 is returned.
    IfNoneMatch(String),
    ///  If the “If-Match” header is given, then it must contain exactly one
    /// Etag. The document is returned, if it has the same revision as the
    /// given Etag. Otherwise a HTTP 412 is returned.
    IfMatch(String),
    NoHeader,
}

impl Default for ReadOptions {
    fn default() -> Self {
        Self::NoHeader
    }
}

/// Options for document removes,
#[derive(Debug, Serialize, Deserialize, TypedBuilder)]
#[builder(doc)]
#[serde(rename_all = "camelCase")]
pub struct RemoveOptions {
    /// Wait until document has been synced to disk.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    wait_for_sync: Option<bool>,
    /// Additionally return the complete old document under the attribute old in
    /// the result.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    return_old: Option<bool>,
    /// If set to true, an empty object will be returned as response.
    /// No meta-data will be returned for the created document.
    /// This option can be used to save some network traffic.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    silent: Option<bool>,
}

impl Default for RemoveOptions {
    fn default() -> Self {
        Self::builder().build()
    }
}
