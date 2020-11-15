//! Document level types
//!
//! This mod contains document related types.
//! Operations are conducted on collection level struct
use serde::{Deserialize, Serialize};
use std::ops::Deref;

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

impl<T> AsRef<T> for Document<T> {
    fn as_ref(&self) -> &T {
        &self.document
    }
}

impl<T> Deref for Document<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.document
    }
}
