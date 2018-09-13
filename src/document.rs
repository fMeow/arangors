use reqwest::Client;
use serde_derive::{Deserialize, Serialize};

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
