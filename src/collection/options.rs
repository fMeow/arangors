//! Types of response related to collection
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

/// Options for checksum
#[derive(Serialize, Deserialize, PartialEq, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ChecksumOptions {
    /// By setting the optional query parameter withRevisions to true, then
    /// revision ids (_rev system attributes) are included in the
    /// checksumming.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    with_revision: Option<bool>,
    /// By providing the optional query parameter withData with a value of true,
    /// the user-defined document attributes will be included in the
    /// calculation too.
    ///
    /// Note: Including user-defined attributes will make
    /// the checksumming slower.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    with_data: Option<bool>,
}

impl Default for ChecksumOptions {
    fn default() -> Self {
        Self::builder().build()
    }
}

#[derive(Debug, Deserialize, Serialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct PropertiesOptions {
    /// If true then creating or changing a document will wait until the data
    /// has been synchronized to disk.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    wait_for_sync: Option<bool>,
    /* TODO need to implement this with feature gate between versions maybe
     *  for ArangoDB 3.7
     * schema: Option<SchemaRules>, */
}
impl Default for PropertiesOptions {
    fn default() -> Self {
        Self::builder().build()
    }
}
