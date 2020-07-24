use serde::{Deserialize, Serialize};

pub mod options;
pub mod response;

#[derive(Serialize, Deserialize, Debug)]
pub struct Header {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub _id: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub _key: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub _rev: String,
}

/// Structure that represents a document within its content and header
#[derive(Serialize, Deserialize, Debug)]
pub struct Document<T> {
    #[serde(flatten)]
    pub header: Header,
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
            header: Header {
                _id: String::new(),
                _key: String::new(),
                _rev: String::new(),
            },
        }
    }
}
