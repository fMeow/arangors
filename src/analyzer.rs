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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum NgramStreamType {
    Binary,
    Utf8,
}

#[derive(Debug, Serialize, Deserialize, TypedBuilder, PartialEq)]
#[builder(doc)]
pub struct DelimiterAnalyzerProperties {
    /// The value will be used as delimiter to split text into tokens as specified
    /// in RFC 4180, without starting new records on newlines.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub delimiter: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, TypedBuilder, PartialEq)]
#[builder(doc)]
pub struct StemAnalyzerProperties {
    /// Format: `language[_COUNTRY][.encoding][@variant]`
    pub locale: String,
}

#[derive(Debug, Serialize, Deserialize, TypedBuilder, PartialEq)]
#[builder(doc)]
pub struct NormAnalyzerProperties {
    /// Format: `language[_COUNTRY][.encoding][@variant]`
    pub locale: String,

    /// Case conversion.  Default: `"lower"`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub case: Option<AnalyzerCase>,

    /// Preserve accents in returned words.  Default: `false`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub accent: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, TypedBuilder, PartialEq)]
#[builder(doc)]
#[serde(rename_all = "camelCase")]
pub struct NgramAnalyzerProperties {
    /// Minimum n-gram length.
    pub min: u16,

    /// Maximum n-gram length.
    pub max: u16,

    /// Output the original value as well.
    pub preserve_original: bool,

    /// Type of the input stream.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub stream_type: Option<NgramStreamType>,
}

#[derive(Debug, Serialize, Deserialize, TypedBuilder, PartialEq)]
#[builder(doc)]
#[serde(rename_all = "camelCase")]
pub struct TextAnalyzerProperties {
    /// Format: `language[_COUNTRY][.encoding][@variant]`
    pub locale: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub case: Option<AnalyzerCase>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub accent: Option<bool>,

    /// Words to omit from result.
    /// Defaults to the words loaded from the file at `stopwordsPath`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub stopwords: Option<Vec<String>>,

    /// Path with a `language` sub-directory containing files with words to omit.
    ///
    /// Defaults to the path specified in the server-side environment variable
    /// IRESEARCH_TEXT_STOPWORD_PATH` or the current working directory of the
    /// ArangoDB process.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub stopwords_path: Option<Vec<String>>,

    /// Apply stemming on returned words.
    /// Default: `true`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub stemming: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum AnalyzerInfo {
    /// The `identity` Analyzer does not take additional properties.
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
