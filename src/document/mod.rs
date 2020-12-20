//! Document level types
//!
//! This mod contains document related types.
//! Operations are conducted on collection level struct
use serde::{de::DeserializeOwned, de::Error as DeError, Deserialize, Deserializer, Serialize};
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
#[derive(Serialize, Debug)]
pub struct Document<T> {
    #[serde(flatten)]
    pub header: Header,
    #[serde(flatten)]
    pub document: T,
}

impl<T> Document<T>
where
    T: Serialize + DeserializeOwned,
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

impl<'de, T> Deserialize<'de> for Document<T>
where
    T: DeserializeOwned,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut obj = serde_json::Value::deserialize(deserializer)?;

        let json = obj
            .as_object_mut()
            .ok_or_else(|| DeError::custom("should be a json object"))?;

        let _id = json
            .get("_id")
            .ok_or_else(|| DeError::missing_field("_id"))?;
        let _key = json
            .get("_key")
            .ok_or_else(|| DeError::missing_field("_key"))?;
        let _rev = json
            .get("_rev")
            .ok_or_else(|| DeError::missing_field("_rev"))?;
        let header: Header = Header {
            _id: serde_json::from_value(_id.clone()).map_err(DeError::custom)?,
            _key: serde_json::from_value(_key.clone()).map_err(DeError::custom)?,
            _rev: serde_json::from_value(_rev.clone()).map_err(DeError::custom)?,
        };
        let document = serde_json::from_value(obj).map_err(DeError::custom)?;

        Ok(Document { header, document })
    }
}
