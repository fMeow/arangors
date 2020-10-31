use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AnalyzerFeature {
    Frequency,
    Norm,
    Position,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AnalyzerCase {
    Lower,
    None,
    Upper,
}

#[derive(Debug, Serialize, Deserialize, TypedBuilder, PartialEq)]
#[builder(doc)]
pub struct DelimiterAnalyzerProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub delimiter: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, TypedBuilder, PartialEq)]
#[builder(doc)]
pub struct StemAnalyzerProperties {
    pub locale: String,
}

#[derive(Debug, Serialize, Deserialize, TypedBuilder, PartialEq)]
#[builder(doc)]
pub struct NormAnalyzerProperties {
    pub locale: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub case: Option<AnalyzerCase>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub accent: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, TypedBuilder, PartialEq)]
#[builder(doc)]
#[serde(rename_all = "camelCase")]
pub struct NgramAnalyzerProperties {
    pub min: u32,

    pub max: u32,

    pub preserve_riginal: bool,
}

#[derive(Debug, Serialize, Deserialize, TypedBuilder, PartialEq)]
#[builder(doc)]
#[serde(rename_all = "camelCase")]
pub struct TextAnalyzerProperties {
    pub locale: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub case: Option<AnalyzerCase>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub accent: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub stopwords: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub stopwords_path: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub stemming: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum AnalyzerInfo {
    Identity {
        name: String,

        #[serde(skip_serializing_if = "Option::is_none")]
        features: Option<Vec<AnalyzerFeature>>,
    },
    Delimiter {
        name: String,

        #[serde(skip_serializing_if = "Option::is_none")]
        features: Option<Vec<AnalyzerFeature>>,

        #[serde(skip_serializing_if = "Option::is_none")]
        properties: Option<DelimiterAnalyzerProperties>,
    },

    Stem {
        name: String,

        #[serde(skip_serializing_if = "Option::is_none")]
        features: Option<Vec<AnalyzerFeature>>,

        #[serde(skip_serializing_if = "Option::is_none")]
        properties: Option<StemAnalyzerProperties>,
    },

    Norm {
        name: String,

        #[serde(skip_serializing_if = "Option::is_none")]
        features: Option<Vec<AnalyzerFeature>>,

        #[serde(skip_serializing_if = "Option::is_none")]
        properties: Option<NormAnalyzerProperties>,
    },

    Ngram {
        name: String,

        #[serde(skip_serializing_if = "Option::is_none")]
        features: Option<Vec<AnalyzerFeature>>,

        #[serde(skip_serializing_if = "Option::is_none")]
        properties: Option<NgramAnalyzerProperties>,
    },

    Text {
        name: String,

        #[serde(skip_serializing_if = "Option::is_none")]
        features: Option<Vec<AnalyzerFeature>>,

        #[serde(skip_serializing_if = "Option::is_none")]
        properties: Option<TextAnalyzerProperties>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnalyzerDescription {
    pub name: String,
}
