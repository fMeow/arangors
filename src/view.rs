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
    pub globally_unique_id: String,

    pub id: String,

    pub name: String,

    #[serde(rename = "type")]
    pub typ: ViewType,
}

#[derive(Debug, Serialize, Deserialize, TypedBuilder)]
#[builder(doc)]
#[serde(rename_all = "camelCase")]
pub struct ArangoSearchViewLink {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub analyzers: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub fields: Option<HashMap<String, ArangoSearchViewLink>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub include_all_fields: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub track_list_positions: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub store_values: Option<StoreValues>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum SortDirection {
    Asc,
    Desc,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum ConsolidationPolicy {
    #[serde(rename_all = "camelCase")]
    BytesAccum { threshold: u32 },

    #[serde(rename_all = "camelCase")]
    Tier {
        #[serde(skip_serializing_if = "Option::is_none")]
        segments_min: Option<u32>,

        #[serde(skip_serializing_if = "Option::is_none")]
        segments_max: Option<u32>,

        #[serde(skip_serializing_if = "Option::is_none")]
        segments_bytes_max: Option<u64>,

        #[serde(skip_serializing_if = "Option::is_none")]
        segments_bytes_floor: Option<u32>,

        min_score: u32,
    },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrimarySort {
    pub field: String,
    pub direction: Option<SortDirection>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StoredValues {
    pub fields: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArangoSearchViewProperties {
    pub cleanup_interval_step: u32,

    pub consolidation_interval_msec: u32,

    pub writebuffer_idle: u32,

    pub writebuffer_active: u32,

    pub writebuffer_size_max: u32,

    pub consolidation_policy: ConsolidationPolicy,

    pub primary_sort: Option<Vec<PrimarySort>>,

    pub primary_sort_compression: PrimarySortCompression,

    pub stored_values: Vec<StoredValues>,

    pub links: HashMap<String, ArangoSearchViewLink>,
}

#[derive(Debug, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ArangoSearchViewPropertiesOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    cleanup_interval_step: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    consolidation_interval_msec: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    writebuffer_idle: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    writebuffer_active: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    writebuffer_size_max: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    consolidation_policy: Option<ConsolidationPolicy>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    primary_sort: Option<Vec<PrimarySort>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    primary_sort_compression: Option<PrimarySortCompression>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    stored_values: Option<Vec<StoredValues>>,

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
