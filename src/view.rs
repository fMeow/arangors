use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use typed_builder::TypedBuilder;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum ViewType {
    #[serde(rename = "arangosearch")]
    ArangoSearchView,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum StoreValues {
    None,
    Id,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PrimarySortCompression {
    Lz4,
    None,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ViewDescription {
    /// A globally unique identifier for this View.
    pub globally_unique_id: String,

    /// An identifier for this View.
    pub id: String,

    /// Name of the View.
    pub name: String,

    /// Type of the View
    #[serde(rename = "type")]
    pub typ: ViewType,
}

#[derive(Debug, Serialize, Deserialize, TypedBuilder)]
#[builder(doc)]
#[serde(rename_all = "camelCase")]
pub struct ArangoSearchViewLink {
    /// A list of names of Analyzers to apply to values of processed document
    /// attributes.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub analyzers: Option<Vec<String>>,

    ///  An object mapping names of attributes to process for each document to
    ///  definitions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub fields: Option<HashMap<String, ArangoSearchViewLink>>,

    /// If set to `true`, all document attributes will be processed, otherwise
    /// only the attributes in `fields` will be processed.
    /// Default: false
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub include_all_fields: Option<bool>,

    /// If set to `true`, the position of values in array values will be tracked,
    /// otherwise all values in an array will be treated as equal alternatives.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub track_list_positions: Option<bool>,

    /// Controls how the view should keep track of the attribute values.
    ///  Default: `"none"`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub store_values: Option<StoreValues>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum SortDirection {
    Asc,
    Desc,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum ConsolidationPolicy {
    #[serde(rename_all = "camelCase")]
    /// Must be in the range of `0.0` to `1.0`.
    BytesAccum { threshold: u32 },

    #[serde(rename_all = "camelCase")]
    Tier {
        /// Minimum number of segments that will be evaluated as candidates
        /// for consolidation.
        /// Default: `1`
        #[serde(skip_serializing_if = "Option::is_none")]
        segments_min: Option<u32>,

        /// Maximum number of segments that will be evaluated as candidates
        /// for consolidation.
        /// Default: `10`
        #[serde(skip_serializing_if = "Option::is_none")]
        segments_max: Option<u32>,

        /// Maximum allowed size of all consolidated segments.
        ///  Default: `5368709120`, i.e. 5 GiB
        #[serde(skip_serializing_if = "Option::is_none")]
        segments_bytes_max: Option<u64>,

        /// Defines the value to treat all smaller segments as equal for
        /// consolidation selection.
        /// Default: `2097152`, i.e. 2 MiB
        #[serde(skip_serializing_if = "Option::is_none")]
        segments_bytes_floor: Option<u32>,

        /// Minimum score.
        min_score: u32,
    },
}

#[derive(Debug, Serialize, Deserialize, TypedBuilder, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PrimarySort {
    /// Attribute path for the value of each document used for
    /// sorting.
    pub field: String,

    /// If set to `"asc"`, the primary sorting order is ascending.
    /// If set to `"desc"`, the primary sorting order is descending.
    #[builder(default, setter(strip_option))]
    direction: Option<SortDirection>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    asc: Option<bool>,
}

impl PrimarySort {
    pub fn direction(&self) -> Option<SortDirection> {
        if self.direction.is_none() {
            if let Some(asc) = self.asc {
                if asc {
                    Some(SortDirection::Asc)
                } else {
                    Some(SortDirection::Desc)
                }
            } else {
                None
            }
        } else {
            self.direction.clone()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StoredValues {
    pub fields: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArangoSearchViewProperties {
    /// How many commits to wait between removing unused files.
    pub cleanup_interval_step: u32,

    /// How long to wait between applying the `consolidationPolicy`.
    pub consolidation_interval_msec: u32,

    /// Maximum number of writers cached in the pool.
    pub writebuffer_idle: u32,

    /// Maximum number of concurrent active writers that perform a transaction.
    pub writebuffer_active: u32,

    /// Maximum memory byte size per writer before a writer flush is triggered.
    pub writebuffer_size_max: u32,

    /// Consolidation policy to apply for selecting which segments should be
    /// merged
    pub consolidation_policy: ConsolidationPolicy,

    /// Attribute paths for which values should be stored in the view index
    /// in addition to those used for sorting via `primary_sort`.
    pub primary_sort: Option<Vec<PrimarySort>>,

    /// Compression to use for the primary sort data.
    /// Default: `"lz4"`
    pub primary_sort_compression: PrimarySortCompression,

    /// Attribute paths for which values should be stored in the view index
    /// in addition to those used for sorting via primary_sort.
    pub stored_values: Vec<StoredValues>,

    /// An object mapping names of linked collections to
    /// ArangoSearchViewLink
    pub links: HashMap<String, ArangoSearchViewLink>,
}

#[derive(Debug, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ArangoSearchViewPropertiesOptions {
    /// How many commits to wait between removing unused files.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    cleanup_interval_step: Option<u32>,

    /// How long to wait between applying the `consolidationPolicy`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    consolidation_interval_msec: Option<u32>,

    /// Maximum number of writers cached in the pool.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    writebuffer_idle: Option<u32>,

    /// Maximum number of concurrent active writers that perform a transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    writebuffer_active: Option<u32>,

    /// Maximum memory byte size per writer before a writer flush is triggered.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    writebuffer_size_max: Option<u32>,

    /// Consolidation policy to apply for selecting which segments should be
    /// merged
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    consolidation_policy: Option<ConsolidationPolicy>,

    /// Attribute paths for which values should be stored in the view index
    /// in addition to those used for sorting via `primary_sort`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    primary_sort: Option<Vec<PrimarySort>>,

    /// Compression to use for the primary sort data.
    /// Default: `"lz4"`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    primary_sort_compression: Option<PrimarySortCompression>,

    // Attribute paths for which values should be stored in the view index
    /// in addition to those used for sorting via primary_sort.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    stored_values: Option<Vec<StoredValues>>,

    /// An object mapping names of linked collections to
    /// ArangoSearchViewLink
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    links: Option<HashMap<String, ArangoSearchViewLink>>,
}

#[derive(Debug, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc)]
pub struct ViewOptions {
    name: String,

    #[serde(rename = "type")]
    #[builder(default=ViewType::ArangoSearchView)]
    typ: ViewType,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    properties: Option<ArangoSearchViewPropertiesOptions>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct View {
    #[serde(flatten)]
    pub description: ViewDescription,

    #[serde(flatten)]
    pub properties: ArangoSearchViewProperties,
}
